use super::category::Category;
use crate::schema::posts;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Associations, Selectable, Identifiable, Clone)]
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

#[derive(Serialize, Deserialize, Clone, Queryable)]
pub struct PostWithCategory {
    #[diesel(sql_type = Integer)]
    pub post_id: i32,
    #[diesel(sql_type = Text)]
    pub post_title: String,
    #[diesel(sql_type = Text)]
    pub post_body: String,
    #[diesel(sql_type = Bool)]
    pub post_published: bool,
    #[diesel(sql_type = Nullable<Integer>)]
    pub post_category_id: Option<i32>,
    #[diesel(sql_type = Integer)]
    pub category_id: Option<i32>,
    #[diesel(sql_type = Text)]
    pub cattegory_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PostCategory {
    pub post: Post,
    pub category: Option<Category>,
}
