use axum::{extract::State, Json};
use sqlx::PgPool;
use crate::models::authors::Author;

pub async fn list_authors(State(pool): State<PgPool>) -> Json<Vec<Author>> {
    let authors = sqlx::query_as::<_, Author>("SELECT * FROM authors")
        .fetch_all(&pool)
        .await
        .unwrap_or_else(|_| vec![]);

    Json(authors)
}
