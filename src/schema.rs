// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "poststatus"))]
    pub struct PostStatus;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PostStatus;

    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        status -> PostStatus,
        col1 -> Int4,
        col2 -> Int4,
    }
}
