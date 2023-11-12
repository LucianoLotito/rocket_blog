use rocket::Route;

use crate::controllers::*;

pub fn routes() -> Vec<Route> {
    let mut all_routes = Vec::new();
    all_routes.extend(post_controller::post_routes());
    all_routes.extend(category_controller::category_routes());
    all_routes
}
