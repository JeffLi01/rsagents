
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard, Arc};
use std::thread::{self, sleep};
use std::time::{Duration, Instant};

use log::{debug, trace};
use rocket::launch;
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
        trace!("in update_service_status thread");
        let manager = managed.read();
        let agent = manager.agent_get_earliest_refreshed().map(|x| x.to_owned());
        drop(manager);
        if agent.is_none() {
            sleep(Duration::new(1, 0));
            continue;
        }
        let now = Instant::now();
        let mut agent = agent.unwrap();
        agent.update_service_status();
        let mut manager = managed.write();
        manager.agent_update_service_status(&agent.info.guid, &agent.services);
        drop(manager);
        let elapsed = now.elapsed();
        debug!("update_service_status used {} milliseconds", elapsed.as_millis());
        if elapsed.as_millis() < 1000 {
            sleep(Duration::new(1, 0) - elapsed);
        }
    }
}


#[launch]
pub fn rocket_app() -> _ {
    env_logger::init();

    let state = Arc::new(RwLock::new(Manager::new()));
    let managed = Managed { state: Arc::clone(&state) };
    thread::spawn(move || update_service_status(managed));
    let managed = Managed { state: Arc::clone(&state) };
    rocket::build()
        .mount("/", api::get_routes())
        .manage(managed)
        .attach(Template::fairing())
}
