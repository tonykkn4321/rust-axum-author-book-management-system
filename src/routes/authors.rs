use axum::{extract::{Path, State}, http::StatusCode, Json};
use sqlx::PgPool;
use crate::models::authors::Author;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateAuthor {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

pub async fn patch_author(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
    Json(payload): Json<UpdateAuthor>,
) -> Result<StatusCode, StatusCode> {
    if payload.first_name.is_none() && payload.last_name.is_none() {
        return Ok(StatusCode::BAD_REQUEST);
    }

    let mut query = String::from("UPDATE authors SET ");
    let mut args: Vec<(String, String)> = vec![];

    if let Some(first_name) = payload.first_name {
        query.push_str("first_name = $1");
        args.push(("first_name".to_string(), first_name));
    }

    if let Some(last_name) = payload.last_name {
        if !args.is_empty() {
            query.push_str(", ");
        }
        query.push_str("last_name = $2");
        args.push(("last_name".to_string(), last_name));
    }

    query.push_str(" WHERE id = $3");

    let result = match args.len() {
        1 => sqlx::query(&query)
            .bind(&args[0].1)
            .bind(id)
            .execute(&pool)
            .await,
        2 => sqlx::query(&query)
            .bind(&args[0].1)
            .bind(&args[1].1)
            .bind(id)
            .execute(&pool)
            .await,
        _ => unreachable!(),
    };

    match result {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
