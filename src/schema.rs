// @generated automatically by Diesel CLI.

diesel::table! {
    snake (id) {
        id -> Int4,
        username -> Text,
        score -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password -> Bytea,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    snake,
    users,
);
