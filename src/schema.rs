use diesel::Insertable;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

diesel::table! {
    connections (id) {
        id -> Text,
        status -> Text,
        removal_datetime -> Nullable<Text>,
    }
}

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = connections)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ConnectionsStruct {
    pub id: String,
    pub status: String,
    pub removal_datetime: Option<String>,
}

diesel::table! {
    details (id, connection_id) {
        id -> Text,
        connection_id -> Text,
        data -> Json,
    }
}

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = details)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct DetailsStruct {
    pub id: String,
    pub connection_id: String,
    pub data: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DetailsJSON {
    pub id: String,
    pub data_type: String,
    pub type_metadata: Value,
    pub confidence: Option<i32>,
    pub interactions: Vec<uuid::Uuid>,
    pub label: String,
    pub data: Value,
}

diesel::table! {
    interactions {
        id -> Text,
        data -> Json,
    }
}

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = interactions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct InteractionsStruct {
    pub id: String,
    pub data: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InteractionJSON {
    pub id: uuid::Uuid,
    pub interaction_type: String,
    pub date_time: String,
    pub interaction_source: InteractionSource,
    pub summary: String,
    pub raw_data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InteractionSource {
    pub file_dir: String,
    pub file_metadata: Value,
}
