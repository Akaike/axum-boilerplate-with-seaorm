use axum::{
    extract::{Path, State},
    Json,
};
use validator::Validate;

use crate::{app::AppState, dto::todo::TodoDto, error::Error, util::validated_json::ValidatedJson};
use serde::Deserialize;
use uuid::Uuid;

pub async fn get_by_id(
    State(AppState { todo_service }): State<AppState>,
    Path(todo_id): Path<Uuid>,
) -> Result<Json<TodoDto>, Error> {
    match todo_service.get_todo_by_id(todo_id).await {
        Ok(todo) => Ok(Json(TodoDto::from(todo))),
        Err(e) => Err(Error::from(e)),
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTodoRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "must be between 1 and 255 characters long"
    ))]
    pub title: String,
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
