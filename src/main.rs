
use std::{sync::{RwLock, RwLockReadGuard, RwLockWriteGuard, Arc}, thread::{self, sleep}, time::Duration};

use rocket::{launch};
use rocket_dyn_templates::Template;

use manager::Manager;

mod api;
mod manager;


pub struct Managed {
    state: Arc<RwLock<Manager>>,
}

impl Managed {
    pub fn read(&self) -> RwLockReadGuard<Manager>
    {
        self.state.read().unwrap()
    }

    pub fn write(&self) -> RwLockWriteGuard<Manager>
    {
        self.state.write().unwrap()
    }
}


fn update_service_status(managed: Managed)
{
    loop {
        println!("in update_service_status thread");
        sleep(Duration::new(10, 0));
        let mut manager = managed.write();
        for agent in manager.agent_get_all_mut() {
            agent.update_service_status();
        }
    }
}


#[launch]
pub fn rocket_app() -> _ {
    let state = Arc::new(RwLock::new(Manager::new()));
    let managed = Managed { state: Arc::clone(&state) };
    thread::spawn(move || update_service_status(managed));
    let managed = Managed { state: Arc::clone(&state) };
    rocket::build()
        .mount("/", api::get_routes())
        .manage(managed)
        .attach(Template::fairing())
}
