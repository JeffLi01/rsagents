use router::Router;
use crate::controllers::agents;

pub fn generate_router() -> Router {
    let mut router = Router::new();
    router.get("/agents", agents::list, "list");
    router.post("/agents", agents::update, "update");
    router
}
