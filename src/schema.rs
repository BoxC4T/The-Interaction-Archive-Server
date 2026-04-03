// @generated automatically by Diesel CLI.

diesel::table! {
    connections (id) {
        id -> Text,
        status -> Text,
        removal_date -> Date,
    }
}

diesel::table! {
    details (id, connection_id) {
        id -> Text,
        connection_id -> Text,
    }
}
