use entity::todo::Model;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize)]
pub struct TodoDto {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
}

impl From<Model> for TodoDto {
    fn from(model: Model) -> Self {
        TodoDto {
            id: model.id,
            title: model.title,
            completed: model.completed,
        }
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

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTodoRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "must be between 1 and 255 characters long"
    ))]
    pub title: String,
    pub completed: bool,
}
