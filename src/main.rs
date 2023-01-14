use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::thread::{self, sleep};
use std::time::{Duration, SystemTime};

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
        let manager = managed.read();
        let agent = manager.agent_get_next_need_refresh().cloned();
        drop(manager);
        if agent.is_none() {
            trace!("update_service_status: no agent to update");
            sleep(Duration::new(1, 0));
            continue;
        }

        let agent = agent.unwrap();
        if agent.age(SystemTime::now()) < 300 {
            trace!("update_service_status: no agent needs update");
            sleep(Duration::new(1, 0));
            continue;
        }

        trace!("update_service_status: refreshing {}", agent.info.guid);
        let mut agent = agent;
        agent.update_service_status();
        let mut manager = managed.write();
        manager.agent_update_service_status(&agent.info.guid, &agent.services);
        drop(manager);
    }
}

#[launch]
pub fn rocket_app() -> _ {
    let _ = env_logger::try_init();

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
