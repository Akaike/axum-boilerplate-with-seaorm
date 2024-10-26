use entity::todo::Model;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
