use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::{repository::todo::TodoRepositoryImpl, service::todo::TodoService};

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
