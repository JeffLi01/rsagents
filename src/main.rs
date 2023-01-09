use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::thread::{self, sleep};
use std::time::Duration;

use log::trace;
use rocket::launch;
use rocket_dyn_templates::Template;

use manager::Manager;

mod api;
mod manager;

pub struct Managed {
    state: Arc<RwLock<Manager>>,
}

impl Managed {
    pub fn read(&self) -> RwLockReadGuard<Manager> {
        self.state.read().unwrap()
    }

    pub fn write(&self) -> RwLockWriteGuard<Manager> {
        self.state.write().unwrap()
    }
}

fn update_service_status(managed: Managed) {
    loop {
        trace!("in update_service_status thread");

        let manager = managed.read();
        let is_empty = manager.waiting_udate.is_empty();
        drop(manager);
        if is_empty {
            sleep(Duration::new(1, 0));
            continue;
        }

        let mut manager = managed.write();
        let guid = manager.waiting_udate.pop().unwrap();
        let agent = manager.agent_get(&guid);
        drop(manager);
        if agent.is_none() {
            sleep(Duration::new(1, 0));
            continue;
        }

        let mut agent = agent.unwrap();
        agent.update_service_status();
        let mut manager = managed.write();
        manager.agent_update_service_status(&agent.info.guid, &agent.services);
        drop(manager);
    }
}

#[launch]
pub fn rocket_app() -> _ {
    env_logger::init();

    let state = Arc::new(RwLock::new(Manager::new()));
    let managed = Managed {
        state: Arc::clone(&state),
    };
    thread::spawn(move || update_service_status(managed));
    let managed = Managed {
        state: Arc::clone(&state),
    };
    rocket::build()
        .mount("/", api::get_routes())
        .manage(managed)
        .attach(Template::fairing())
}
