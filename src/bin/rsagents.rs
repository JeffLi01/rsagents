use iron::Plugin;
use iron::prelude::*;
use iron::status;
use iron::typemap::Key;
use mime::*;
use persistent::{State};
use router::Router;
use urlencoded::UrlEncodedBody;

#[derive(Debug)]
pub struct Agent {
    name: String,
    ip: String,
    bmc_ip: String,
}

#[derive(Copy, Clone)]
pub struct HitCounter;

impl Key for HitCounter { type Value = Vec<Agent>; }

fn main() {
    let mut router = Router::new();
    router.get("/agents", list, "list");
    router.post("/agents", update, "update");

    let mut chain = Chain::new(router);
    let agents: Vec<Agent> = Vec::new();
    chain.link(State::<HitCounter>::both(agents));

    println!("Serving on http://localhost:3000...");
    Iron::new(chain).http("localhost:3000").unwrap();
}

fn list(req: &mut Request) -> IronResult<Response> {
    let rwlock = req.get::<State<HitCounter>>().unwrap();
    let agents = rwlock.read().unwrap();

    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    let mut content = "<title>Agents</title>".to_string();
    content.push_str(r#"<table border="1">"#);
    let part = format!("<tr><th>{}</th><th>{}</th><th>{}</th><th>{}</th></tr>", "Index", "Name", "IP", "BMC IP");
    content.push_str(&part);
    for index in 0 .. agents.len() {
        let agent = &agents[index];
        let part = format!("<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>", index, agent.name, agent.ip, agent.bmc_ip);
        content.push_str(&part);
    }
    content.push_str("</table>");
    response.set_mut(content);

    Ok(response)
}

fn update(req: &mut Request) -> IronResult<Response> {
    let rwlock = req.get::<State<HitCounter>>().unwrap();
    let mut agents = rwlock.write().unwrap();

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

    let agent = Agent{
        name: name,
        ip: ip,
        bmc_ip: bmc_ip,
    };
    agents.push(agent);
    Ok(Response::with((status::Ok, format!("Agents: {:#?}", *agents))))
}