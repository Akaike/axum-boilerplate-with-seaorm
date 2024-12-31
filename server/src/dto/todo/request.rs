use serde::Deserialize;
use validator::Validate;

use crate::validator::todo::validate_title_length;

/// Represents the data needed to create a new todo
#[derive(Debug, Deserialize, Validate)]
pub struct CreateTodo {
    /// The title of the todo item
    #[validate(custom(function = "validate_title_length"))]
    pub title: String,
}

/// Represents the data needed to update an existing todo
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTodo {
    /// The new title for the todo item
    #[validate(custom(function = "validate_title_length"))]
    pub title: String,
    /// Whether the todo is completed
    pub completed: bool,
}
