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
    pub data_type: DataTypes,
    pub type_metadata: Option<DetailTypeMetadata>,
    pub confidence: Option<i32>,
    pub interactions: Option<Vec<uuid::Uuid>>,
    pub label: Option<String>,
    pub data: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DataTypes {
    String,
    StringArray,
    FormatedString,
    FormatedStringArray,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DetailTypeMetadata {
    StringArrayMetadata(StringArrayMetadata),
    FormatedStringMetadata(FormatedStringMetadata),
    FormatedStringArrayMetadata(FormatedStringArrayMetadata),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StringArrayMetadata {
    pub length: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FormatedStringMetadata {
    pub regex: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FormatedStringArrayMetadata {
    pub length: Option<u32>,
    pub regex: String,
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
    pub date_time: Option<String>,
    pub interaction_source: Option<InteractionSource>,
    pub summary: Option<String>,
    pub raw_data: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InteractionSource {
    pub file_dir: String,
    pub file_metadata: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Regex {
    SingleRegex(SingleRegex),
    MultiRegex(MultiRegex),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SingleRegex {
    regex: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MultiRegex {
    regex: Vec<String>,
}
