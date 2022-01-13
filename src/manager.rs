use std::time::{SystemTime, Duration};
use std::str::FromStr;
use std::net::{SocketAddr, TcpStream};

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
pub struct ServiceStatus {
    pub http: bool,
    pub https: bool,
    pub ssh: bool,
    pub ipmi: bool,
    pub vnc: bool,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Agent {
    pub info: AgentInfo,
    pub timestamp: SystemTime,
    pub duration_s: u64,
    pub services: ServiceStatus,
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
    println!("{}: {:?}", addr, res);
    match res {
        Ok(_) => return true,
        Err(_) => return false,
    }
}

impl Agent {
    pub fn update_service_status(&mut self)
    {
        let duration = SystemTime::now().duration_since(self.timestamp).ok().unwrap().as_secs();
        if duration > 310 {
            self.services = Default::default();
            return;
        }
        self.services.http = is_port_on(&self.info.bmc_ip, 80);
        self.services.https = is_port_on(&self.info.bmc_ip, 443);
        self.services.ssh = is_port_on(&self.info.bmc_ip, 22);
        self.services.ipmi = is_port_on(&self.info.bmc_ip, 623);
        self.services.vnc = is_port_on(&self.info.bmc_ip, 5900);
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
        let agent = Agent {
            info: agent_info,
            timestamp: SystemTime::now(),
            services: Default::default(),
            duration_s: 0,
        };
        if let Some(_) = self.agent_get(&agent.info.guid) {
            self.agent_delete(&agent.info.guid);
        }
        self.agents.insert(0, agent);
        self.agents[0].clone()
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

    pub fn agent_delete(&mut self, guid: &str)
    {
        if let Some(index) = self.agents.iter().position(|agent| &agent.info.guid == guid) {
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
