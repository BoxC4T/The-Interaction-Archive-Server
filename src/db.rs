use axum::{Json, http::StatusCode};
use diesel::prelude::*;
use diesel::{Connection, Insertable, sqlite::SqliteConnection};
use serde::{Deserialize, Serialize};

extern crate uuid;
use serde_json::Value;

use crate::file_handler;
use crate::schema::connections;

pub fn establish_connection() -> SqliteConnection {
    let data_dir = file_handler::get_data_dir();
    let db_path = file_handler::get_db_file(&data_dir);
    let database_url = format!("sqlite://{}", db_path.display());
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Detail {
    pub id: String,
    pub data_type: String,
    pub type_metadata: DetailTypeMetadata,
    pub confidence: i32,
    pub interactions: Vec<uuid::Uuid>,
    pub data: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DetailTypeMetadata {
    pub array_length: u32,
    pub regex: SingleOrArray,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Interaction {
    pub id: uuid::Uuid,
    pub interaction_type: String,
    pub date_time: String,
    pub interaction_source: InteractionSource,
    pub summary: String,
    pub raw_data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InteractionSource {
    file_dir: String,
    file_metadata: FileMetadata,
}

#[derive(Serialize, Deserialize, Debug)]
struct FileMetadata {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum SingleOrArray {
    SingleString(SingleString),
    ArrayString(ArrayString),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SingleString(String);

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrayString(Vec<String>);

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = connections)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewConnection {
    pub id: String,
    pub status: String,
}

pub async fn create_new_connection() -> (StatusCode, Json<String>) {
    let conn = &mut establish_connection();
    let con_uuid = uuid::Uuid::new_v4();

    let fresh_connection = NewConnection {
        id: con_uuid.to_string(),
        status: "active".to_string(),
    };
    diesel::insert_into(connections::table)
        .values(fresh_connection)
        .returning(NewConnection::as_returning())
        .get_result(conn)
        .unwrap();

    //REMINDER TO FIX STATUS CODES LATER
    (StatusCode::CREATED, Json(con_uuid.to_string()))
}

// pub async fn get_all_connections(
//     Json(payload): Json<ConnTypeHeader>,
// ) -> (StatusCode, Json<Vec<String>>) {
//     let conn = &mut establish_connection();
// }

struct ConnTypeHeader {
    conn_type: String,
}
