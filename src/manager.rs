use std::time::SystemTime;
use serde_derive::{Deserialize, Serialize};

use rocket::form::FromForm;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Agent {
    pub guid: String,
    pub name: String,
    pub ip: String,
    pub bmc_ip: String,
    pub timestamp: SystemTime,
}

#[derive(FromForm)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewAgent {
    pub guid: String,
    pub name: String,
    pub ip: String,
    pub bmc_ip: String,
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
    pub fn agent_create(&mut self, new_agent: NewAgent) -> Agent
    {
        let agent = Agent {
            guid: new_agent.guid,
            name: new_agent.name,
            ip: new_agent.ip,
            bmc_ip: new_agent.bmc_ip,
            timestamp: SystemTime::now(),
        };
        if let Some(_) = self.agent_get(&agent.guid) {
            self.agent_delete(&agent.guid);
        }
        self.agents.insert(0, agent);
        self.agents[0].clone()
    }

    pub fn agent_get_all(&self) -> Vec<Agent>
    {
        self.agents.clone()
    }
    
    pub fn agent_get(&self, guid: &str) -> Option<Agent>
    {
        self.agents.iter().find(|agent| agent.guid == guid).map(|x| x.to_owned())
    }

    pub fn agent_delete(&mut self, guid: &str)
    {
        if let Some(index) = self.agents.iter().position(|agent| &agent.guid == guid) {
            self.agents.remove(index);
        }
    }
}
