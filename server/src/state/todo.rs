use crate::{repository::todo::TodoRepositoryImpl, service::todo::TodoService};
use axum::extract::FromRef;

use super::app::AppState;

#[derive(Clone)]
pub struct TodoState {
    pub todo_service: TodoService<TodoRepositoryImpl>,
}

impl FromRef<AppState> for TodoState {
    fn from_ref(app_state: &AppState) -> Self {
        Self {
            todo_service: app_state.todo_service.clone(),
        }
    }
}
