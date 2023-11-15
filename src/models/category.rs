use crate::schema::categories;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Selectable, Identifiable, Clone)]
#[diesel(table_name = categories)]
pub struct Category {
    #[diesel(sql_type = BigInteger)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub name: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = categories)]

pub struct NewCategory {
    pub name: String,
}
