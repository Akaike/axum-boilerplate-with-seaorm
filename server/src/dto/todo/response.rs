use entity::todo::Model;
use serde::Serialize;
use uuid::Uuid;

/// Represents a todo item in API responses
#[derive(Serialize)]
pub struct Todo {
    /// Unique identifier for the todo
    pub id: Uuid,
    /// The title of the todo item
    pub title: String,
    /// Whether the todo is completed
    pub completed: bool,
}

impl From<Model> for Todo {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            title: model.title,
            completed: model.completed,
        }
    }
}
