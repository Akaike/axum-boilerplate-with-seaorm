use entity::todo::Model;

use std::sync::Arc;
use uuid::Uuid;

use crate::common::error::ServiceResult;

use super::repository::TodoRepository;

#[derive(Clone)]
pub struct TodoService<R: TodoRepository> {
    pub repo: Arc<R>,
}

impl<R: TodoRepository> TodoService<R> {
    pub fn new(repo: Arc<R>) -> Self {
        TodoService { repo }
    }

    pub async fn get_todo_by_id(&self, id: Uuid) -> ServiceResult<Model> {
        Ok(self.repo.get_by_id(id).await?)
    }

    pub async fn create_todo(&self, title: String) -> ServiceResult<Model> {
        Ok(self.repo.create(title).await?)
    }

    pub async fn update_todo(
        &self,
        id: Uuid,
        title: String,
        completed: bool,
    ) -> ServiceResult<Model> {
        Ok(self.repo.update(id, title, completed).await?)
    }

    pub async fn delete_todo(&self, id: Uuid) -> ServiceResult<()> {
        Ok(self.repo.delete(id).await?)
    }
}
