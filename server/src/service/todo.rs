use entity::todo::Model as TodoModel;
use sea_orm::DbErr;

use crate::repository::todo::TodoRepository;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct TodoService<R: TodoRepository> {
    pub repo: Arc<R>,
}

impl<R: TodoRepository> TodoService<R> {
    pub fn new(repo: Arc<R>) -> Self {
        TodoService { repo }
    }

    pub async fn get_todo_by_id(&self, id: Uuid) -> Result<TodoModel, DbErr> {
        self.repo.get_by_id(id).await
    }

    pub async fn create_todo(&self, title: String) -> Result<TodoModel, DbErr> {
        self.repo.create(title).await
    }
}
