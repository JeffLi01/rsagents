use std::net::{SocketAddr, TcpStream};
use std::str::FromStr;
use std::time::Duration;

use chrono::{DateTime, Local};
use log::debug;
use serde::Deserialize;
use serde_derive::Serialize;
use serde_json::{self, json};

use rayon::prelude::*;

use rsagents::{Error, Load, Store};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AgentInfo {
    pub guid: String,
    pub name: String,
    pub ip: String,
    pub bmc_ip: String,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Service {
    pub name: String,
    pub port: u16,
    pub alive: bool,
}

impl Service {
    pub fn new(name: String, port: u16) -> Self {
        Self {
            name,
            port,
            alive: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Agent {
    pub info: AgentInfo,
    pub create_time: DateTime<Local>,
    pub services: Vec<Service>,
    pub last_refresh: Option<DateTime<Local>>,
}

fn is_port_on(ip: &str, port: u16) -> bool {
    let addr_str = format!("{}:{}", ip, port);
    let result = SocketAddr::from_str(&addr_str);
    let addr = match result {
        Ok(addr) => addr,
        Err(_) => return false,
    };
    let res = TcpStream::connect_timeout(&addr, Duration::new(1, 0));
    debug!("{}: {:?}", addr, res);
    res.is_ok()
}

impl Agent {
    pub fn new(agent_info: AgentInfo) -> Self {
        let mut agent = Agent {
            info: agent_info,
            create_time: Local::now(),
            services: vec![],
            last_refresh: None,
        };
        for (name, port) in vec![
            ("http", 80),
            ("https", 443),
            ("ssh", 22),
            ("ipmi", 623),
            ("vnc", 5900),
        ] {
            agent.monitor_service(name, port);
        }
        agent
    }

    fn monitor_service(&mut self, name: &str, port: u16) {
        self.services.push(Service::new(name.to_string(), port));
    }

    fn clear_service_status(&mut self) {
        for service in &mut self.services {
            service.alive = false;
        }
    }

    pub fn update_service_status(&mut self) {
        self.clear_service_status();
        self.services
            .par_iter_mut()
            .for_each(|service| service.alive = is_port_on(&self.info.bmc_ip, service.port));
    }

    pub fn age(&self, now: DateTime<Local>) -> Option<i64> {
        self.last_refresh.map(|x| (now - x).num_seconds())
    }
}

#[cfg(test)]
mod test {
    use chrono::Local;

    use super::{AgentInfo, Agent};

    #[test]
    fn test_age() {
        let info = AgentInfo {
            guid: "guid".into(),
            name: "name".into(),
            ip: "ip".into(),
            bmc_ip: "bmc_ip".into(),
        };
        let mut agent = Agent::new(info);
        let now = Local::now();
        assert_eq!(agent.age(now), None);
        agent.last_refresh = Some(now);
        assert_eq!(agent.age(now), Some(0));
    }
}

#[derive(Clone, Default)]
pub struct Manager {
    pub agents: Vec<Agent>,
    pub refresh_interval_s: i64,
}

impl Manager {
    pub fn new() -> Self {
        Self {
            refresh_interval_s: 300,
            ..Default::default()
        }
    }

    pub fn load(&mut self, storage: &impl Load) -> Result<(), Error> {
        let s = storage.load()?;
        let agent_infos: Vec<AgentInfo> = serde_json::from_str(&s)?;
        for agent_info in agent_infos {
            let _ = self.agent_create(agent_info);
        }
        Ok(())
    }

    pub fn store(&self, storage: &impl Store) -> Result<(), Error> {
        let agent_infos: Vec<AgentInfo> = self.agents.iter().map(|x| x.info.clone()).collect();
        storage.store(&serde_json::to_string_pretty(&json!(agent_infos))?)?;
        Ok(())
    }
}

impl Manager {
    pub fn agent_create(&mut self, agent_info: AgentInfo) -> Agent {
        let agent = Agent::new(agent_info);
        let mut i = 0;
        while i < self.agents.len() {
            if self.agents[i].info.guid == agent.info.guid
                && self.agents[i].info.name == agent.info.name
            {
                self.agents.remove(i);
                break;
            } else {
                i += 1;
            }
        }
        self.agents.push(agent);
        self.agents.last().unwrap().clone()
    }

    pub fn agent_update_service_status(&mut self, guid: &str, services: &[Service]) {
        if let Some(agent) = self.agents.iter_mut().find(|agent| agent.info.guid == guid) {
            agent.services = services.to_owned();
            agent.last_refresh = Some(Local::now());
        }
    }

    pub fn agent_get_all(&self) -> Vec<Agent> {
        self.agents.clone()
    }

    pub fn agent_get_all_mut(&mut self) -> Vec<&mut Agent> {
        self.agents.iter_mut().collect()
    }

    pub fn agent_get(&self, guid: &str) -> Option<Agent> {
        self.agents
            .iter()
            .find(|agent| agent.info.guid == guid)
            .map(|x| x.to_owned())
    }

    pub fn agent_needs_refresh(&self, agent: &Agent, now: DateTime<Local>) -> bool {
        self.agent_priority(agent, now) > self.refresh_interval_s
    }

    pub fn agent_get_next_need_refresh(&self) -> Option<&Agent> {
        let now = Local::now();
        self.agents
            .iter()
            .filter(|agent| self.agent_needs_refresh(agent, now))
            .max_by_key(|agent| self.agent_priority(agent, now))
    }

    pub fn agent_delete(&mut self, guid: &str) {
        if let Some(index) = self.agents.iter().position(|agent| agent.info.guid == guid) {
            self.agents.remove(index);
        }
    }

    pub fn update_service_status(&mut self) {
        self.agents
            .par_iter_mut()
            .for_each(|agent| agent.update_service_status());
    }

    pub fn agent_priority(&self, agent: &Agent, now: DateTime<Local>) -> i64 {
        match agent.age(now) {
            Some(age) => age,
            None => {
                let elapsed_since_created = now - agent.create_time;
                elapsed_since_created.num_seconds() + self.refresh_interval_s
            }
        }
    }
}
