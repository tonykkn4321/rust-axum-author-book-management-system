use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber;
use dotenvy::dotenv;

mod db;
mod models;
mod routes;

use crate::db::connect_db;
use crate::routes::authors::list_authors;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let pool = connect_db().await.expect("Failed to connect to DB");

    let app = Router::new()
        .route("/authors", get(list_authors))
        .with_state(pool);

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await.unwrap();

    tracing::info!("Listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}
