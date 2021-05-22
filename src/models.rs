use std::time::{Duration, SystemTime};
use iron::typemap::Key;

#[derive(Clone, Debug)]
pub struct Agent {
    pub guid: String,
    pub name: String,
    pub ip: String,
    pub bmc_ip: String,
    pub timestamp: SystemTime,
}

#[derive(Clone, Debug)]
pub struct Agents {
    agents: Vec<Agent>,
}

#[derive(Clone)]
pub struct AgentsKey;

impl Key for AgentsKey { type Value = Agents; }


impl Agents {
    pub fn new() -> Self
    {
        let agents: Vec<Agent> = vec![];
        Agents{agents}
    }
    pub fn to_html(&self) -> String
    {
        let now = SystemTime::now();
        let mut content = "<title>Agents</title>".to_string();
        content.push_str(r#"<meta http-equiv="refresh" content="10">"#);
        content.push_str(r#"<table border="1">"#);
        content.push_str("<tr>");
        content.push_str(format!("<th>{}</th>", "Index").as_ref());
        content.push_str(format!("<th>{}</th>", "GUID").as_ref());
        content.push_str(format!("<th>{}</th>", "Name").as_ref());
        content.push_str(format!("<th>{}</th>", "IP").as_ref());
        content.push_str(format!("<th>{}</th>", "BMC IP").as_ref());
        content.push_str(format!("<th>{}</th>", "Last Update").as_ref());
        content.push_str("</tr>");
        for index in 0 .. self.agents.len() {
            let agent = &self.agents[index];
            content.push_str("<tr>");
            content.push_str(format!("<td>{}</td>", index).as_ref());
            content.push_str(format!("<td>{}</td>", agent.guid).as_ref());
            content.push_str(format!("<td>{}</td>", agent.name).as_ref());
            content.push_str(format!("<td>{}</td>", agent.ip).as_ref());
            content.push_str(format!("<td>{}</td>", agent.bmc_ip).as_ref());
            let mut bg_color = "#ff0000";
            let duration = match now.duration_since(agent.timestamp) {
                Ok(duration) => {
                    if duration.as_secs() < 20 * 60 {
                        bg_color = "#00cc00";
                    }
                    format!("{} ago", readable_duration(&duration))
                }
                Err(_) => format!("SystemTime before last update")
            };
            content.push_str(format!(r#"<td bgcolor="{}"">{}</td>"#, bg_color, duration).as_ref());
            content.push_str("</tr>");
        }
        content.push_str("</table>");
        content
    }
    pub fn update(&mut self, new_agent: &Agent)
    {
        let mut old_index = 0;
        for index in 0 .. self.agents.len() {
            let agent = &self.agents[index];
            if agent.guid == new_agent.guid {
                break;
            }
            old_index += 1;
        }
        if old_index < self.agents.len() {
            self.agents.remove(old_index);
        }
        self.agents.insert(0, new_agent.clone());
    }
}

pub fn readable_duration(duration: &Duration) -> String
{
    let mut seconds = duration.as_secs();
    let mut minutes = seconds / 60;
    seconds %= 60;
    let mut hours = minutes / 60;
    minutes %= 60;
    let days = hours / 24;
    hours %= 24;

    let mut content = String::new();
    if days > 0 {
        content.push_str(format!("{}d", days).as_ref());
    }
    if hours > 0 {
        content.push_str(format!("{}h", hours).as_ref());
    }
    if minutes > 0 {
        content.push_str(format!("{}m", minutes).as_ref());
    }
    if seconds > 0 || content.len() == 0 {
        content.push_str(format!("{}s", seconds).as_ref());
    }
    content
}
