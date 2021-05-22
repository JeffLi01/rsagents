use iron::prelude::*;
use persistent::{State};

use rsagents::{Agents, AgentsKey, generate_router};

fn main() {
    let router = generate_router();

    let mut chain = Chain::new(router);
    let agents = Agents::new();
    chain.link(State::<AgentsKey>::both(agents));

    println!("Serving on http://localhost:3000/agents..., DO NOT CLOSE");
    Iron::new(chain).http("0.0.0.0:3000").unwrap();
}
