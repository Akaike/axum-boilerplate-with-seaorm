use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, NoContent},
    Json,
};
use tracing::error;
use uuid::Uuid;

use crate::{
    common::error::ApiResult, common::state::AppState, common::validated_json::ValidatedJson,
    common::validated_path::ValidatedPath,
};

use super::model::{CreateTodoRequest, TodoResponse, UpdateTodoRequest};

pub async fn get_by_id(
    State(state): State<AppState>,
    ValidatedPath(todo_id): ValidatedPath<Uuid>,
) -> ApiResult<Json<TodoResponse>> {
    let todo = state
        .todo_service
        .get_todo_by_id(todo_id)
        .await
        .map_err(|err| {
            error!(todo_id = %todo_id, "Failed to get todo by id: {:?}", err);
            err
        })?;

    Ok(Json(TodoResponse::from(todo)))
}

pub async fn create(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<CreateTodoRequest>,
) -> ApiResult<impl IntoResponse> {
    let todo = state
        .todo_service
        .create_todo(payload.title.clone())
        .await
        .map_err(|err| {
            error!(title = %payload.title, "Failed to create todo: {:?}", err);
            err
        })?;

    Ok((StatusCode::CREATED, Json(TodoResponse::from(todo))))
}

pub async fn update(
    State(state): State<AppState>,
    ValidatedPath(todo_id): ValidatedPath<Uuid>,
    ValidatedJson(payload): ValidatedJson<UpdateTodoRequest>,
) -> ApiResult<Json<TodoResponse>> {
    let todo = state
        .todo_service
        .update_todo(todo_id, payload.title.clone(), payload.completed)
        .await
        .map_err(|err| {
            error!(todo_id = %todo_id, title = %payload.title, completed = %payload.completed, "Failed to update todo: {:?}", err);
            err
        })?;

    Ok(Json(TodoResponse::from(todo)))
}

pub async fn delete(
    State(state): State<AppState>,
    ValidatedPath(todo_id): ValidatedPath<Uuid>,
) -> ApiResult<NoContent> {
    state
        .todo_service
        .delete_todo(todo_id)
        .await
        .map_err(|err| {
            error!(todo_id = %todo_id, "Failed to delete todo: {:?}", err);
            err
        })?;

    Ok(NoContent)
}

