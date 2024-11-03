use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::{
    app::AppState,
    dto::todo::{CreateTodoRequest, TodoDto, UpdateTodoRequest},
    error::Error,
    util::validated_json::ValidatedJson,
};

pub async fn get_by_id(
    State(AppState { todo_service }): State<AppState>,
    Path(todo_id): Path<Uuid>,
) -> Result<Json<TodoDto>, Error> {
    match todo_service.get_todo_by_id(todo_id).await {
        Ok(todo) => Ok(Json(TodoDto::from(todo))),
        Err(e) => Err(Error::from(e)),
    }
}

pub async fn create(
    State(AppState { todo_service }): State<AppState>,
    ValidatedJson(payload): ValidatedJson<CreateTodoRequest>,
) -> Result<Json<TodoDto>, Error> {
    match todo_service.create_todo(payload.title).await {
        Ok(todo) => Ok(Json(TodoDto::from(todo))),
        Err(e) => Err(Error::from(e)),
    }
}

pub async fn update(
    State(AppState { todo_service }): State<AppState>,
    Path(todo_id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<UpdateTodoRequest>,
) -> Result<Json<TodoDto>, Error> {
    match todo_service
        .update_todo(todo_id, payload.title, payload.completed)
        .await
    {
        Ok(todo) => Ok(Json(TodoDto::from(todo))),
        Err(e) => Err(Error::from(e)),
    }
}
