use axum::{
    Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber;
use dotenvy::dotenv;
use axum::routing::{get, post, put, patch, delete};

mod db;
mod models;
mod routes;

use crate::db::connect_db;
use crate::routes::authors::{
    list_authors, create_author, replace_author, patch_author, delete_author,
};

#[tokio::main]
async fn main() {
    // Load environment variables from .env
    dotenv().ok();

    // Initialize structured logging
    tracing_subscriber::fmt::init();

    // Connect to PostgreSQL database
    let pool = connect_db().await.expect("Failed to connect to DB");

    // Define application routes
    let app = Router::new()
        .route("/authors", get(list_authors).post(create_author))
        .route("/authors/:id", put(replace_author).patch(patch_author).delete(delete_author))
        .with_state(pool);

    // Determine port from environment or default to 3000
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await.unwrap();

    tracing::info!("🚀 Server running at http://{}", addr);

    // Start the Axum server
    axum::serve(listener, app).await.unwrap();
}
