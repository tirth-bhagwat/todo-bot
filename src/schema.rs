// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Int4,
        title -> Text,
        description -> Nullable<Text>,
        user_id -> Int4,
        status -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Text,
        telegram_id -> Text,
    }
}

diesel::joinable!(todos -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    todos,
    users,
);
