use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::thread::{self, sleep};
use std::time::Duration;

use log::trace;
use rocket::fairing::AdHoc;
use rocket::launch;
use rocket_dyn_templates::Template;

use manager::Manager;
use rsagents::{Load, Error, Store};

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
        trace!("update_service_status: refreshing {}", agent.info.guid);
        let mut agent = agent;
        agent.update_service_status();
        let mut manager = managed.write();
        manager.agent_update_service_status(&agent.info.guid, &agent.services);
        drop(manager);
    }
}

struct LocalStorage {
    path: PathBuf,
}

impl Load for LocalStorage {
    fn load(&self) -> Result<String, Error> {
        let content = fs::read_to_string(&self.path)?;
        Ok(content)
    }
}

impl Store for LocalStorage {
    fn store(&self, content: &str) -> Result<(), Error> {
        match self.path.parent() {
            Some(dir) => {
                if !dir.exists() {
                    fs::create_dir_all(dir)?;
                }
            },
            None => {},
        }
        let mut file = File::create(&self.path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}

#[launch]
pub fn rocket_app() -> _ {
    let _ = env_logger::try_init();

    let mut path = dirs::config_dir().unwrap();
    path.push("rsagents");
    path.push("agents.json");
    let storage = LocalStorage { path };
    let mut manager = Manager::new();
    match manager.load(&storage) {
        Ok(_) => {},
        Err(err) => {
            eprintln!("{:?}", err);
        },
    }
    let state = Arc::new(RwLock::new(manager));
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
        .attach(AdHoc::on_shutdown("Store agents", |inst| Box::pin(async move {
            let managed = inst.state::<Managed>().unwrap();
            match managed.read().store(&storage) {
                Ok(_) => {},
                Err(err) => {
                    eprintln!("{:?}", err);
                }
            }
        })))
}
