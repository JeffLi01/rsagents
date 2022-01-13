use std::time::SystemTime;

use rocket::{get, post};
use rocket::{routes, Route, State};
use rocket::form::Form;
use rocket::serde::json::Json;
use serde_derive::{Deserialize, Serialize};

use rocket_dyn_templates::Template;

use crate::Managed;
use crate::manager::{Agent, AgentInfo};


#[post("/agents", data = "<agent>")]
pub fn api_agent_create(agent: Form<AgentInfo>, state: &State<Managed>) -> Json<Agent>
{
    let mut managed = state.write();
    Json(managed.agent_create(agent.into_inner()))
}

#[get("/agents", format = "application/json")]
pub fn api_agent_list(state: &State<Managed>) -> Json<Vec<Agent>>
{
    let managed = state.read();
    Json(managed.agent_get_all())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Info {
    agents: Vec<Agent>,
}

#[get("/agents", format = "text/html", rank = 2)]
pub fn api_agent_list_html(state: &State<Managed>) -> Template
{
    let managed = state.read();
    let mut agents = managed.agent_get_all();
    for agent in &mut agents {
        agent.duration_s = SystemTime::now().duration_since(agent.timestamp).ok().unwrap().as_secs();
    }
    Template::render("agents", &Info { agents })
}

pub fn get_routes() -> Vec<Route>
{
    routes![
        api_agent_create,
        api_agent_list,
        api_agent_list_html,
    ]
}
