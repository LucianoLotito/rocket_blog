use crate::database::establish_connection_pg;
use crate::models::category::Category;
use crate::models::post::*;
use crate::schema::*;
extern crate diesel;
use diesel::result::Error;
use diesel::{ExpressionMethods, NullableExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::response::{status::Accepted, status::Created, Debug};
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, routes};
type Result<T, E = Debug<Error>> = std::result::Result<T, E>;
use rocket::Route;

pub fn post_routes() -> Vec<Route> {
    routes![create, retrieve, update, delete, post_category]
}

#[post("/post", format = "json", data = "<post>")]
fn create(post: Json<NewPost>) -> Result<Created<Json<Post>>> {
    let connection = &mut establish_connection_pg();

    let new_post = NewPost {
        title: post.title.to_string(),
        body: post.body.to_string(),
        published: post.published,
        category_id: post.category_id,
    };

    let result_post = diesel::insert_into(crate::schema::posts::dsl::posts)
        .values(&new_post)
        .get_result::<crate::models::post::Post>(connection)
        .expect("Error saving new post");
    Ok(Created::new("/").body(Json(result_post)))
}

#[get("/post/<id>")]
fn retrieve(id: i32) -> Result<Accepted<Json<Post>>> {
    match crate::schema::posts::table
        .find(id)
        .first(&mut establish_connection_pg())
    {
        Ok(post) => Ok(Accepted(Json(post))),
        Err(err) => Err(Debug(err)),
    }
}

#[put("/post/<id>", format = "json", data = "<new_post_data>")]
fn update(id: i32, new_post_data: Json<NewPost>) -> Result<Accepted<Json<Post>>> {
    let connection = &mut establish_connection_pg();
    let post = diesel::update(crate::schema::posts::dsl::posts)
        .set((
            posts::title.eq(&new_post_data.title),
            posts::body.eq(&new_post_data.body),
            posts::published.eq(&new_post_data.published),
            posts::category_id.eq(&new_post_data.category_id),
        ))
        .filter(posts::id.eq(id))
        .get_result::<crate::models::post::Post>(connection)
        .expect("Error updating post");
    Ok(Accepted(Json(post)))
}

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Aldo {
    status: String,
}

#[delete("/post/<id>")]
fn delete(id: i32) -> Result<Accepted<Json<Aldo>>> {
    let connection = &mut establish_connection_pg();
    match diesel::delete(crate::schema::posts::dsl::posts)
        .filter(posts::id.eq(id))
        .execute(connection)
    {
        Ok(_post) => Ok(Accepted(Json(Aldo {
            status: String::from("Post has been deleted"),
        }))),
        Err(err) => Err(Debug(err)),
    }
}

#[get("/post/<id>/category")]
fn post_category(id: i32) -> Result<Accepted<Json<PostCategory>>> {
    let results: Vec<PostWithCategory> = posts::table
        .left_join(categories::table)
        .select((
            posts::id,
            posts::title,
            posts::body,
            posts::published,
            posts::category_id.nullable(),
            categories::id.nullable(),
            categories::name.nullable(),
        ))
        .filter(posts::id.eq(id))
        .load(&mut establish_connection_pg())
        .expect("Error loading results");

    match results.first() {
        Some(result) => Ok(Accepted(Json(PostCategory {
            post: Post {
                id: result.post_id,
                title: result.post_title.to_string(),
                body: result.post_body.to_string(),
                published: result.post_published,
                category_id: result.post_category_id,
            },
            category: match (result.category_id, result.cattegory_name.clone()) {
                (Some(id), Some(name)) => Some(Category { id, name }),
                (_, _) => None,
            },
        }))),
        None => Err(Debug(Error::NotFound)),
    }
}
