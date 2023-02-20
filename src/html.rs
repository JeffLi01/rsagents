use rocket::{get, Route, routes};

pub fn get_routes() -> Vec<Route> {
    routes![
        html_icon,
    ]
}

#[get("/favicon.ico", format = "image/*")]
pub fn html_icon() -> Option<&'static [u8]> {
    Some(include_bytes!("images/favicon.ico"))
}
