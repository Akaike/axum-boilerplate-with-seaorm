use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::{
    app::AppState,
    dtos::todo::{CreateTodoRequest, TodoDto, UpdateTodoRequest},
    error::Error,
    utils::validated_json::ValidatedJson,
};

pub async fn get_by_id(
    State(state): State<AppState>,
    Path(todo_id): Path<Uuid>,
) -> Result<Json<TodoDto>, Error> {
    let todo = state.todo_service.get_todo_by_id(todo_id).await?;

    Ok(Json(TodoDto::from(todo)))
}

pub async fn create(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<CreateTodoRequest>,
) -> Result<Json<TodoDto>, Error> {
    let todo = state.todo_service.create_todo(payload.title).await?;

    Ok(Json(TodoDto::from(todo)))
}

pub async fn update(
    State(state): State<AppState>,
    Path(todo_id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<UpdateTodoRequest>,
) -> Result<Json<TodoDto>, Error> {
    let todo = state
        .todo_service
        .update_todo(todo_id, payload.title, payload.completed)
        .await?;

    Ok(Json(TodoDto::from(todo)))
}
