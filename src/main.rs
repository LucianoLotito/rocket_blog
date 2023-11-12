extern crate diesel;
extern crate rocket;
use rocket::launch;
use rocket_dyn_templates::Template;
use routes::api::routes;
mod controllers;
mod database;
pub mod models;
mod routes;
pub mod schema;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes())
        .attach(Template::fairing())
}
