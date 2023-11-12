use crate::schema::categories;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Selectable, Identifiable)]
#[diesel(table_name = categories)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = categories)]

pub struct NewCategory {
    pub name: String,
}
