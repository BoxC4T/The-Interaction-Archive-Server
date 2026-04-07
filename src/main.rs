use axum::{
    Router,
    routing::{get, post},
};

use crate::file_handler::Config;

mod db;
mod file_handler;
mod schema;

#[tokio::main]
async fn main() {
    let config: Config = file_handler::load_config().unwrap();

    tracing_subscriber::fmt::init();

    let app = Router::new().route("/connections/", post(db::create_new_connection));
    // .route("/connections/", get(db::));

    let mut api_addr: String = "localhost:".to_string();
    api_addr += &config.api_config.port.to_string();
    let listener = tokio::net::TcpListener::bind(&api_addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
