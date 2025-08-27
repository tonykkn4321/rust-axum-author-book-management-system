use axum::{
    routing::{get, post, put, patch, delete},
    Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber;
use dotenvy::dotenv;

mod db;
mod models;
mod routes;

use crate::db::connect_db;
use crate::routes::authors::{
    list_authors, create_author, replace_author, patch_author, delete_author,
};

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let pool = connect_db().await.expect("Failed to connect to DB");

    let app = Router::new()
        .route("/authors", get(list_authors).post(create_author))
        .route("/authors/:id", put(replace_author).patch(patch_author).delete(delete_author))
        .with_state(pool);

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await.unwrap();

    tracing::info!("🚀 Server running at http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
