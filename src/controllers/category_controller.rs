use crate::database::establish_connection_pg;
use crate::models::category::*;
extern crate diesel;
use diesel::RunQueryDsl;
use rocket::response::{status::Created, Debug};
use rocket::serde::json::Json;
use rocket::{post, routes, Route};
type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

pub fn category_routes() -> Vec<Route> {
    routes![create]
}

#[post("/category", format = "json", data = "<category>")]
pub fn create(category: Json<NewCategory>) -> Result<Created<Json<Category>>> {
    let connection = &mut establish_connection_pg();

    let new_category = NewCategory {
        name: category.name.to_string(),
    };

    let result_category = diesel::insert_into(crate::schema::categories::dsl::categories)
        .values(&new_category)
        .get_result::<crate::models::category::Category>(connection)
        .expect("Error saving new category");
    Ok(Created::new("/").body(Json(result_category)))
}
