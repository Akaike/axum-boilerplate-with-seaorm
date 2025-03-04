use entity::todo::Model;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use validator::ValidationError;

#[derive(Serialize)]
pub struct TodoResponse {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
}

impl From<Model> for TodoResponse {
    fn from(model: Model) -> Self {
        Self {
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

pub fn validate_title_length(title: &str) -> Result<(), ValidationError> {
    if title.len() < 1 as usize || title.len() > 255 as usize {
        return Err(ValidationError::new(
            "must be between 1 and 255 characters long",
        ));
    }
    Ok(())
}


