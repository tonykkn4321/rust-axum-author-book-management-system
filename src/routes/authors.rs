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
pub async fn list_authors(State(pool): State<PgPool>) -> Result<Json<Vec<Author>>, StatusCode> {
    let authors = sqlx::query_as::<_, Author>("SELECT * FROM authors")
        .fetch_all(&pool)
        .await
        .map_err(|err| {
            tracing::error!("Database query failed: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(authors))
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
    sqlx::query("UPDATE authors SET first_name = $1, last_name = $2 WHERE id = $3")
        .bind(&payload.first_name)
        .bind(&payload.last_name)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

// PATCH /authors/:id
pub async fn patch_author(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
    Json(payload): Json<UpdateAuthor>,
) -> Result<StatusCode, StatusCode> {
    let mut query = String::from("UPDATE authors SET ");
    let mut args = vec![];
    let mut param_index = 1;

    if let Some(first_name) = &payload.first_name {
        query.push_str(&format!("first_name = ${}", param_index));
        args.push(first_name.clone());
        param_index += 1;
    }

    if let Some(last_name) = &payload.last_name {
        if !args.is_empty() {
            query.push_str(", ");
        }
        query.push_str(&format!("last_name = ${}", param_index));
        args.push(last_name.clone());
        param_index += 1;
    }

    if args.is_empty() {
        return Ok(StatusCode::BAD_REQUEST);
    }

    query.push_str(&format!(" WHERE id = ${}", param_index));

    let mut sql = sqlx::query(&query);
    for arg in &args {
        sql = sql.bind(arg);
    }
    sql = sql.bind(id);

    sql.execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

// DELETE /authors/:id
pub async fn delete_author(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
) -> Result<StatusCode, StatusCode> {
    sqlx::query("DELETE FROM authors WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}
