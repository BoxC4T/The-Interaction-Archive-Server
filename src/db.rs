use axum::{Json, extract::Query, http::StatusCode};
use diesel::prelude::*;
use diesel::{Connection, sqlite::SqliteConnection};
use serde::Deserialize;

extern crate uuid;

use crate::file_handler;
use crate::schema;

pub fn establish_connection() -> SqliteConnection {
    let data_dir = file_handler::get_data_dir();
    let db_path = file_handler::get_db_file(&data_dir);
    let database_url = format!("sqlite://{}", db_path.display());
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub async fn create_new_connection() -> Result<(StatusCode, Json<String>), StatusCode> {
    let conn = &mut establish_connection();
    let con_uuid = uuid::Uuid::new_v4();

    let fresh_connection = schema::ConnectionsStruct {
        id: con_uuid.to_string(),
        status: "active".to_string(),
        removal_datetime: None,
    };
    let db_res = diesel::insert_into(schema::connections::table)
        .values(fresh_connection)
        .returning(schema::ConnectionsStruct::as_returning())
        .get_result(conn);

    match db_res {
        Ok(result) => Ok((StatusCode::CREATED, Json(result.id))),
        Err(e) => {
            eprintln!("Insert error: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_connections(
    Query(payload): Query<ConnTypeHeader>,
) -> Result<(StatusCode, Json<Vec<String>>), StatusCode> {
    let conn = &mut establish_connection();

    match payload.conn_type {
        None => {
            let db_res = schema::connections::dsl::connections
                .filter(schema::connections::dsl::status.eq("active"))
                .load::<schema::ConnectionsStruct>(conn);

            match db_res {
                Ok(result) => Ok((
                    StatusCode::ACCEPTED,
                    Json(result.iter().map(|f| f.id.clone()).collect()),
                )),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Some(conn_type) => {
            //marked - marked for removal
            let allowed = ["active", "archived", "marked"];

            if allowed.contains(&conn_type.as_str()) {
                let db_res = schema::connections::dsl::connections
                    .filter(schema::connections::dsl::status.eq(conn_type))
                    .load::<schema::ConnectionsStruct>(conn);

                match db_res {
                    Ok(result) => Ok((
                        StatusCode::ACCEPTED,
                        Json(result.iter().map(|f| f.id.clone()).collect()),
                    )),
                    Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
                }
            } else {
                Err(StatusCode::BAD_REQUEST)
            }
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct ConnTypeHeader {
    #[serde(default)]
    conn_type: Option<String>,
}
