use std::path::Path;

use rocket::{fs::NamedFile, get, Route, routes};

pub fn get_routes() -> Vec<Route> {
    routes![
        html_icon,
    ]
}

#[get("/favicon.ico", format = "image/*")]
pub async fn html_icon() -> Option<NamedFile> {
    NamedFile::open(Path::new("favicon.ico")).await.ok()
}
