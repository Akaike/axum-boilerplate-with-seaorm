use entity::todo::Model;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
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
