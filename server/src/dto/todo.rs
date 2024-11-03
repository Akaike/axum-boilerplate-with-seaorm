use entity::todo::Model;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::validators::todo::validate_title_length;

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
    #[validate(custom(function = "validate_title_length"))]
    pub title: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTodoRequest {
    #[validate(custom(function = "validate_title_length"))]
    pub title: String,
    pub completed: bool,
}
