use super::category::Category;
use crate::schema::posts;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Associations, Selectable, Identifiable)]
#[diesel(belongs_to(Category, foreign_key= category_id))]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub category_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub published: bool,
    pub category_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct PostWithCategory {
    pub id: i32,
    pub title: String,
    pub published: bool,
    pub body: String,
    pub category_id: Option<i32>,
    pub category: Option<Category>,
}
