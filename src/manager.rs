use std::time::{SystemTime, Duration};
use std::str::FromStr;
use std::net::{SocketAddr, TcpStream};

use log::debug;
use serde_derive::{Deserialize, Serialize};

use rocket::form::FromForm;
use rayon::prelude::*;


#[derive(FromForm)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgentInfo {
    pub guid: String,
    pub name: String,
    pub ip: String,
    pub bmc_ip: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub port: u16,
    pub alive: bool,
}

impl Service {
    pub fn new(name: String, port: u16) -> Self { Self { name, port, alive: false } }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Agent {
    pub info: AgentInfo,
    pub create_time: SystemTime,
    pub duration_s: u64,
    pub services: Vec<Service>,
    pub service_refresh_time: SystemTime,
}

fn is_port_on(ip: &str, port: u16) -> bool
{
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
    pub fn new(agent_info: AgentInfo) -> Self
    {
        let services = vec![
            Service::new("http".to_string(), 80),
            Service::new("https".to_string(), 443),
            Service::new("ssh".to_string(), 22),
            Service::new("ipmi".to_string(), 623),
            Service::new("vnc".to_string(), 5900),
            ];
        Agent {
            info: agent_info,
            create_time: SystemTime::now(),
            services,
            duration_s: 0,
            service_refresh_time: SystemTime::now(),
        }
    }

    fn clear_service_status(&mut self)
    {
        for service in &mut self.services {
            service.alive = false;
        }
    }
    pub fn update_service_status(&mut self)
    {
        let duration = SystemTime::now().duration_since(self.create_time).ok().unwrap().as_secs();
        if duration > 310 {
            self.clear_service_status();
            return;
        }
        self.services.par_iter_mut()
            .for_each(|service| {
                service.alive = is_port_on(&self.info.bmc_ip, service.port)
            });
    }
}


#[derive(Clone, Default, Serialize)]
pub struct Manager {
    pub agents: Vec<Agent>,
}

impl Manager {
    pub fn new() -> Self
    {
        Self { ..Default::default() }
    }
}

impl Manager {
    pub fn agent_create(&mut self, agent_info: AgentInfo) -> Agent
    {
        let agent = Agent::new(agent_info);
        let mut i = 0;
        while i < self.agents.len() {
            if self.agents[i].info.guid == agent.info.guid && self.agents[i].info.name == agent.info.name {
                self.agents.remove(i);
            } else {
                i += 1;
            }
        }
        self.agents.insert(0, agent);
        self.agents[0].clone()
    }

    pub fn agent_update_service_status(&mut self, guid: &str, services: &[Service])
    {
        if let Some(agent) = self.agents.iter_mut().find(|agent| agent.info.guid == guid) {
            agent.services = services.to_owned();
            agent.service_refresh_time = SystemTime::now();
        }
    }

    pub fn agent_get_all(&self) -> Vec<Agent>
    {
        self.agents.clone()
    }

    pub fn agent_get_all_mut(&mut self) -> Vec<&mut Agent>
    {
        self.agents.iter_mut().collect()
    }
    
    pub fn agent_get(&self, guid: &str) -> Option<Agent>
    {
        self.agents.iter().find(|agent| agent.info.guid == guid).map(|x| x.to_owned())
    }
    
    pub fn agent_get_earliest_refreshed(&self) -> Option<&Agent>
    {
        self.agents.iter().min_by_key(|agent| agent.service_refresh_time)
    }

    pub fn agent_delete(&mut self, guid: &str)
    {
        if let Some(index) = self.agents.iter().position(|agent| agent.info.guid == guid) {
            self.agents.remove(index);
        }
    }

    pub fn update_service_status(&mut self)
    {
        self.agents.par_iter_mut()
            .for_each(|agent| {
                agent.update_service_status()
            });
    }
}
