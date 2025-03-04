use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, NoContent},
    Json,
};
use uuid::Uuid;

use crate::{
    common::error::Error, common::state::AppState, common::validated_json::ValidatedJson
};

use super::model::{CreateTodoRequest, TodoResponse, UpdateTodoRequest};

pub async fn get_by_id(
    State(state): State<AppState>,
    Path(todo_id): Path<Uuid>,
) -> Result<Json<TodoResponse>, Error> {
    let todo = state.todo_service.get_todo_by_id(todo_id).await?;

    Ok(Json(TodoResponse::from(todo)))
}

pub async fn create(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<CreateTodoRequest>,
) -> Result<impl IntoResponse, Error> {
    let todo = state.todo_service.create_todo(payload.title).await?;

    Ok((StatusCode::CREATED, Json(TodoResponse::from(todo))))
}

pub async fn update(
    State(state): State<AppState>,
    Path(todo_id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<UpdateTodoRequest>,
) -> Result<Json<TodoResponse>, Error> {
    let todo = state
        .todo_service
        .update_todo(todo_id, payload.title, payload.completed)
        .await?;

    Ok(Json(TodoResponse::from(todo)))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(todo_id): Path<Uuid>,
) -> Result<NoContent, Error> {
    state.todo_service.delete_todo(todo_id).await?;

    Ok(NoContent)
}
