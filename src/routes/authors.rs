use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use crate::models::authors::Author;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewAuthor {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize)]
pub struct UpdateAuthor {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

// GET /authors
pub async fn list_authors(State(pool): State<PgPool>) -> Json<Vec<Author>> {
    let authors = sqlx::query_as::<_, Author>("SELECT * FROM authors")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    Json(authors)
}

// POST /authors
pub async fn create_author(
    State(pool): State<PgPool>,
    Json(payload): Json<NewAuthor>,
) -> Result<StatusCode, StatusCode> {
    sqlx::query("INSERT INTO authors (first_name, last_name) VALUES ($1, $2)")
        .bind(&payload.first_name)
        .bind(&payload.last_name)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}

// PUT /authors/:id
pub async fn replace_author(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
    Json(payload): Json<NewAuthor>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("UPDATE authors SET first_name = $1, last_name = $2 WHERE id = $3")
        .bind(&payload.first_name)
        .bind(&payload.last_name)
        .bind(id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// PATCH /authors/:id
pub async fn patch_author(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
    Json(payload): Json<UpdateAuthor>,
) -> Result<StatusCode, StatusCode> {
    let mut query = String::from("UPDATE authors SET ");
    let mut updates = vec![];
    let mut binds: Vec<&(dyn sqlx::Encode<'_> + sqlx::Type<Postgres>)> = vec![];

    if let Some(first_name) = &payload.first_name {
        updates.push("first_name = $1");
        binds.push(first_name);
    }
    if let Some(last_name) = &payload.last_name {
        updates.push("last_name = $2");
        binds.push(last_name);
    }

    if updates.is_empty() {
        return Ok(StatusCode::NO_CONTENT);
    }

    query.push_str(&updates.join(", "));
    query.push_str(" WHERE id = $3");

    let result = sqlx::query(&query)
        .bind(binds.get(0))
        .bind(binds.get(1))
        .bind(id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// DELETE /authors/:id
pub async fn delete_author(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM authors WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
