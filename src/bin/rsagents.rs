use iron::Plugin;
use iron::prelude::*;
use iron::status;
use iron::typemap::Key;
use mime::*;
use persistent::{State};
use router::Router;
use urlencoded::UrlEncodedBody;
use std::time::{Duration, SystemTime};

#[derive(Clone, Debug)]
pub struct Agent {
    guid: String,
    name: String,
    ip: String,
    bmc_ip: String,
    timestamp: SystemTime,
}

#[derive(Clone, Debug)]
pub struct Agents {
    agents: Vec<Agent>,
}

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

#[derive(Clone)]
pub struct AgentsKey;

impl Key for AgentsKey { type Value = Agents; }

fn main() {
    let mut router = Router::new();
    router.get("/agents", list, "list");
    router.post("/agents", update, "update");

    let mut chain = Chain::new(router);
    let agents = Agents::new();
    chain.link(State::<AgentsKey>::both(agents));

    println!("Serving on http://localhost:3000/agents..., DO NOT CLOSE");
    Iron::new(chain).http("0.0.0.0:3000").unwrap();
}

fn list(req: &mut Request) -> IronResult<Response> {
    let rwlock = req.get::<State<AgentsKey>>().unwrap();
    let agents = &*rwlock.read().unwrap();

    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    let content = agents.to_html();
    response.set_mut(content);

    Ok(response)
}

fn update(req: &mut Request) -> IronResult<Response> {
    let rwlock = req.get::<State<AgentsKey>>().unwrap();
    let mut rwlock_agents = rwlock.write().unwrap();

    let mut response = Response::new();

    let form_data = match req.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing form data: {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => map
    };

    let name = match form_data.get("name") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("form data has no 'name' parameter\n"));
            return Ok(response);
        }
        Some(names) => names[0].clone()
    };

    let guid = match form_data.get("guid") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("form data has no 'guid' parameter\n"));
            return Ok(response);
        }
        Some(guids) => guids[0].clone()
    };

    let ip = match form_data.get("ip") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("form data has no 'ip' parameter\n"));
            return Ok(response);
        }
        Some(ips) => ips[0].clone()
    };

    let bmc_ip = match form_data.get("bmc_ip") {
        None => "".to_string(),
        Some(bmc_ips) => bmc_ips[0].clone()
    };

    let timestamp = SystemTime::now();
    let new_agent = Agent{guid, name, ip, bmc_ip, timestamp};
    let mut old_index = 0;
    for index in 0 .. rwlock_agents.agents.len() {
        let agent = &rwlock_agents.agents[index];
        if agent.guid == new_agent.guid {
            break;
        }
        old_index += 1;
    }
    if old_index < rwlock_agents.agents.len() {
        rwlock_agents.agents.remove(old_index);
    }
    rwlock_agents.agents.insert(0, new_agent);
    Ok(Response::with((status::Ok, format!("Agents: {:#?}", rwlock_agents.agents))))
}
