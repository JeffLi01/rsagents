
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
        let now = Instant::now();
        let mut manager = managed.write();
        manager.update_service_status();
        let elapsed = now.elapsed();
        debug!("update_service_status used {} milliseconds", elapsed.as_millis());
        sleep(Duration::new(10, 0) - elapsed);
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
