use iron::Plugin;
use iron::prelude::*;
use iron::status;
use mime::*;
use persistent::{State};
use urlencoded::UrlEncodedBody;
use std::time::{SystemTime};

use crate::{Agent, AgentsKey};

pub fn list(req: &mut Request) -> IronResult<Response> {
    let rwlock = req.get::<State<AgentsKey>>().unwrap();
    let agents = &*rwlock.read().unwrap();

    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    let content = agents.to_html();
    response.set_mut(content);

    Ok(response)
}

pub fn update(req: &mut Request) -> IronResult<Response> {
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
    rwlock_agents.update(&new_agent);
    Ok(Response::with((status::Ok, format!("Agents: {:#?}", *rwlock_agents))))
}
