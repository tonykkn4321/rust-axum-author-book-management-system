use axum::{extract::{Path, State}, http::StatusCode, Json};
use sqlx::PgPool;
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

pub async fn list_authors(State(_pool): State<PgPool>) -> Json<Vec<String>> {
    Json(vec!["Author A".to_string(), "Author B".to_string()])
}

pub async fn create_author(State(_pool): State<PgPool>, Json(_payload): Json<NewAuthor>) -> StatusCode {
    StatusCode::CREATED
}

pub async fn replace_author(Path(_id): Path<i32>, State(_pool): State<PgPool>, Json(_payload): Json<NewAuthor>) -> StatusCode {
    StatusCode::OK
}

pub async fn patch_author(Path(_id): Path<i32>, State(_pool): State<PgPool>, Json(_payload): Json<UpdateAuthor>) -> StatusCode {
    StatusCode::OK
}

pub async fn delete_author(Path(_id): Path<i32>, State(_pool): State<PgPool>) -> StatusCode {
    StatusCode::NO_CONTENT
}
