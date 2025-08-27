use axum::{routing::get, Router};
use dotenvy::dotenv;
use std::env;
use std::net::SocketAddr;
use tracing_subscriber;
use rust_axum_author_book_management_system::db::connect_db;
use rust_axum_author_book_management_system::routes::authors::list_authors;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let pool = connect_db().await.expect("Failed to connect to DB");

    let app = Router::new()
        .route("/authors", get(list_authors))
        .with_state(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
