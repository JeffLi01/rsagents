
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use rocket::{launch};
use rocket_dyn_templates::Template;

use manager::Manager;

mod api;
mod manager;


pub struct Managed {
    state: RwLock<Manager>,
}

impl Managed {
    pub fn new() -> Self
    {
        let state: RwLock<Manager> = RwLock::new(Manager::new());
        Self { state }
    }

    pub fn read(&self) -> RwLockReadGuard<Manager>
    {
        self.state.read().unwrap()
    }

    pub fn write(&self) -> RwLockWriteGuard<Manager>
    {
        self.state.write().unwrap()
    }
}


#[launch]
pub fn rocket_app() -> _ {
    rocket::build()
        .mount("/", api::get_routes())
        .manage(Managed::new())
        .attach(Template::fairing())
}
