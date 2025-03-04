use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, NoContent},
    Json,
};
use uuid::Uuid;

use crate::{
    common::error::ApiResult,
    common::state::AppState,
    common::validated_json::ValidatedJson
};

use super::model::{CreateTodoRequest, TodoResponse, UpdateTodoRequest};

pub async fn get_by_id(
    State(state): State<AppState>,
    Path(todo_id): Path<Uuid>,
) -> ApiResult<Json<TodoResponse>> {
    let todo = state.todo_service.get_todo_by_id(todo_id).await?;

    Ok(Json(TodoResponse::from(todo)))
}

pub async fn create(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<CreateTodoRequest>,
) -> ApiResult<impl IntoResponse> {
    let todo = state.todo_service.create_todo(payload.title).await?;

    Ok((StatusCode::CREATED, Json(TodoResponse::from(todo))))
}

pub async fn update(
    State(state): State<AppState>,
    Path(todo_id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<UpdateTodoRequest>,
) -> ApiResult<Json<TodoResponse>> {
    let todo = state
        .todo_service
        .update_todo(todo_id, payload.title, payload.completed)
        .await?;

    Ok(Json(TodoResponse::from(todo)))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(todo_id): Path<Uuid>,
) -> ApiResult<NoContent> {
    state.todo_service.delete_todo(todo_id).await?;

    Ok(NoContent)
}
