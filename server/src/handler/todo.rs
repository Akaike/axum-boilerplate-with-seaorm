use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, NoContent},
    Json,
};
use uuid::Uuid;

use crate::{
    dto::todo::{CreateTodo, Todo, UpdateTodo},
    error::Error,
    state::todo::TodoState,
    util::validated_json::ValidatedJson,
};

pub async fn get_by_id(
    State(state): State<TodoState>,
    Path(todo_id): Path<Uuid>,
) -> Result<Json<Todo>, Error> {
    let todo = state.todo_service.get_todo_by_id(todo_id).await?;

    Ok(Json(Todo::from(todo)))
}

pub async fn create(
    State(state): State<TodoState>,
    ValidatedJson(payload): ValidatedJson<CreateTodo>,
) -> Result<impl IntoResponse, Error> {
    let todo = state.todo_service.create_todo(payload.title).await?;

    Ok((StatusCode::CREATED, Json(Todo::from(todo))))
}

pub async fn update(
    State(state): State<TodoState>,
    Path(todo_id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<UpdateTodo>,
) -> Result<Json<Todo>, Error> {
    let todo = state
        .todo_service
        .update_todo(todo_id, payload.title, payload.completed)
        .await?;

    Ok(Json(Todo::from(todo)))
}

pub async fn delete(
    State(state): State<TodoState>,
    Path(todo_id): Path<Uuid>,
) -> Result<NoContent, Error> {
    state.todo_service.delete_todo(todo_id).await?;

    Ok(NoContent)
}
