use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};

use crate::{app::AppState, dto::todo::TodoDto, error::Error};
use serde::Deserialize;
use uuid::Uuid;

pub async fn get_by_id(
    State(state): State<Arc<AppState>>,
    Path(todo_id): Path<Uuid>,
) -> Result<Json<TodoDto>, Error> {
    match state.todo_service.get_todo_by_id(todo_id).await {
        Ok(todo) => Ok(Json(TodoDto::from(todo))),
        Err(e) => Err(Error::from(e)),
    }
}

#[derive(Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
}

pub async fn create(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTodoRequest>,
) -> Result<Json<TodoDto>, Error> {
    match state.todo_service.create_todo(payload.title).await {
        Ok(todo) => Ok(Json(TodoDto::from(todo))),
        Err(e) => Err(Error::from(e)),
    }
}
