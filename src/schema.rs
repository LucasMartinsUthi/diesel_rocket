// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        login -> Varchar,
        password -> Varchar,
    }
}
