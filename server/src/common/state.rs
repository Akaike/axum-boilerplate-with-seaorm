use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::todo::{repository::TodoRepositoryImpl, service::TodoService};

#[derive(Clone)]
pub struct AppState {
    pub todo_service: TodoService<TodoRepositoryImpl>,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        let todo_repo = Arc::new(TodoRepositoryImpl { db });
        let todo_service = TodoService::new(todo_repo);

        Self { todo_service }
    }
}
