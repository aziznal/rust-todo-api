use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use crate::Context;

#[derive(Serialize, Debug)]
struct TodoResponse {
    message: String,
}

#[derive(Deserialize, Debug)]
struct CreateTodoDto {
    pub name: String,
}

#[derive(Deserialize, Debug)]
struct UpdateTodoDto {
    pub name: String,
}

pub async fn get_all_todos(State(state): State<Context>) -> impl IntoResponse {
    dbg!("TODO: Get All Todos");

    (StatusCode::OK, Json("Got All Todos!"))
}

pub async fn get_todo(
    State(state): State<Context>,
    Path(todo_id): Path<String>,
) -> impl IntoResponse {
    dbg!("TODO: Get Todo by Id");

    (StatusCode::OK, Json("Got Todo!"))
}

pub async fn create_todo(State(state): State<Context>) -> impl IntoResponse {
    dbg!("TODO: Create Todo");

    (StatusCode::OK, Json("Created Todo!"))
}

pub async fn update_todo(
    State(state): State<Context>,
    Path(todo_id): Path<String>,
) -> impl IntoResponse {
    dbg!("TODO: Update Todo");

    (StatusCode::OK, Json("Updated Todo!"))
}

pub async fn delete_todo(
    State(state): State<Context>,
    Path(todo_id): Path<String>,
) -> impl IntoResponse {
    dbg!("TODO: Delete Todo");

    (StatusCode::OK, Json("Deleted Todo!"))
}
