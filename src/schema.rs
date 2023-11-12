// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        body -> Text,
        published -> Bool,
        category_id -> Nullable<Int4>,
    }
}

diesel::joinable!(posts -> categories (category_id));

diesel::allow_tables_to_appear_in_same_query!(categories, posts,);
