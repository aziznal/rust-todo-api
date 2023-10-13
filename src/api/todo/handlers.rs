use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use crate::Context;

#[derive(Serialize, Debug)]
struct TodoResponse {
    message: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateTodoDto {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdateTodoDto {
    pub name: String,
}

async fn get_all_todos(State(state): State<Context>) -> impl IntoResponse {
    dbg!("TODO: Get All Todos");

    (StatusCode::OK, Json("Got All Todos!"))
}

async fn get_todo(State(state): State<Context>, Path(todo_id): Path<String>) -> impl IntoResponse {
    dbg!("TODO: Get Todo by Id");

    (StatusCode::OK, Json("Got Todo!"))
}

async fn create_todo(State(state): State<Context>) -> impl IntoResponse {
    dbg!("TODO: Create Todo");

    (StatusCode::OK, Json("Created Todo!"))
}

async fn update_todo(
    State(state): State<Context>,
    Path(todo_id): Path<String>,
) -> impl IntoResponse {
    dbg!("TODO: Update Todo");

    (StatusCode::OK, Json("Updated Todo!"))
}

async fn delete_todo(
    State(state): State<Context>,
    Path(todo_id): Path<String>,
) -> impl IntoResponse {
    dbg!("TODO: Delete Todo");

    (StatusCode::OK, Json("Deleted Todo!"))
}

pub fn create_router(context: &Context) -> Router {
    Router::new()
        .nest(
            "/todo",
            Router::new()
                .route("/", get(get_all_todos).post(create_todo))
                .nest(
                    "/:id",
                    Router::new().route("/", get(get_todo).put(update_todo).delete(delete_todo)),
                ),
        )
        .with_state(context.to_owned())
}
