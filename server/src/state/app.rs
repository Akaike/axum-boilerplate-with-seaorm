use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::{repositories::todo::TodoRepositoryImpl, services::todo::TodoService};

#[derive(Clone)]
pub struct AppState {
    pub todo_service: TodoService<TodoRepositoryImpl>,
}

pub trait AppStateExt {
    fn todo_service(&self) -> &TodoService<TodoRepositoryImpl>;
}

impl AppStateExt for AppState {
    fn todo_service(&self) -> &TodoService<TodoRepositoryImpl> {
        &self.todo_service
    }
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        let todo_repo = Arc::new(TodoRepositoryImpl { db });
        let todo_service = TodoService::new(todo_repo);

        Self { todo_service }
    }
}
