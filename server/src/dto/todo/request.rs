use serde::Deserialize;
use validator::Validate;

use crate::validator::todo::validate_title_length;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTodo {
    #[validate(custom(function = "validate_title_length"))]
    pub title: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTodo {
    #[validate(custom(function = "validate_title_length"))]
    pub title: String,
    pub completed: bool,
}
