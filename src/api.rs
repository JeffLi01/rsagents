use std::time::SystemTime;

use rocket::form::Form;
use rocket::serde::json::Json;
use rocket::{get, post, FromForm};
use rocket::{routes, Route, State};
use serde_derive::{Deserialize, Serialize};

use rocket_dyn_templates::Template;

use crate::manager::Agent;
use crate::manager::AgentInfo as CoreAgentInfo;
use crate::Managed;

#[derive(FromForm, Serialize, Deserialize)]
pub struct AgentInfo {
    pub guid: String,
    pub name: String,
    pub ip: String,
    pub bmc_ip: String,
}

impl Into<CoreAgentInfo> for AgentInfo {
    fn into(self) -> CoreAgentInfo {
        CoreAgentInfo {
            guid: self.guid,
            name: self.name,
            ip: self.ip,
            bmc_ip: self.bmc_ip,
        }
    }
}

#[post("/agents", data = "<agent>")]
pub fn api_agent_create(agent: Form<AgentInfo>, state: &State<Managed>) -> Json<Agent> {
    let mut managed = state.write();
    Json(managed.agent_create(agent.into_inner().into()))
}

#[post("/agents", data = "<agent>", format = "application/json", rank = 2)]
pub fn api_agent_create_with_json(agent: Json<AgentInfo>, state: &State<Managed>) -> Json<Agent> {
    let mut managed = state.write();
    Json(managed.agent_create(agent.into_inner().into()))
}

#[get("/agents", format = "application/json")]
pub fn api_agent_list(state: &State<Managed>) -> Json<Vec<Agent>> {
    let managed = state.read();
    Json(managed.agent_get_all())
}

#[derive(Clone, Debug, Serialize)]
struct Info {
    agents: Vec<Agent>,
}

#[get("/agents", format = "text/html", rank = 2)]
pub fn api_agent_list_html(state: &State<Managed>) -> Template {
    let managed = state.read();
    let mut agents = managed.agent_get_all();
    for agent in &mut agents {
        agent.duration_s = SystemTime::now()
            .duration_since(agent.create_time)
            .ok()
            .unwrap()
            .as_secs();
    }
    Template::render("agents", &Info { agents })
}

pub fn get_routes() -> Vec<Route> {
    routes![
        api_agent_create,
        api_agent_create_with_json,
        api_agent_list,
        api_agent_list_html,
    ]
}

#[cfg(test)]
mod test {
    use super::AgentInfo;
    use crate::rocket_app;
    use rocket::local::blocking::Client;
    use rocket::http::{Status, ContentType};
    use rocket::uri;
    use serde_json::json;

    #[test]
    fn api_agent_create_with_form() {
        let client = Client::tracked(rocket_app()).expect("valid rocket instance");
        let response = client
            .post(uri!(super::api_agent_create))
            .header(ContentType::Form)
            .body("guid=guid&name&name&ip&ip&bmc_ip&bmc_ip")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn api_agent_create_with_json() {
        let client = Client::tracked(rocket_app()).expect("valid rocket instance");
        let info = AgentInfo {
            guid: "guid".to_owned(),
            name: "name".to_owned(),
            ip: "ip".to_owned(),
            bmc_ip: "bmc_ip".to_owned(),
        };
        let response = client
            .post(uri!(super::api_agent_create))
            .json(&json!(info))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
