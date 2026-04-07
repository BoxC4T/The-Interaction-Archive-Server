diesel::table! {
    connections (id) {
        id -> Text,
        status -> Text,
        removal_datetime -> Nullable<Text>,
    }
}

diesel::table! {
    details (id, connection_id) {
        id -> Text,
        connection_id -> Text,
        data -> Json,
    }
}
diesel::table! {
    interactions {
        id -> Text,
        data -> Json,
    }
}
