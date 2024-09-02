// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Varchar,
        phone -> Varchar,
        address -> Varchar,
    }
}
