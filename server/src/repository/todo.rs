use chrono::{DateTime, Utc};
use entity::todo::{self, Entity as TodoEntity, Model};
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

use async_trait::async_trait;

use crate::error::DbError;

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> Result<todo::Model, DbError>;
    async fn create(&self, title: String) -> Result<todo::Model, DbError>;
    async fn update(
        &self,
        id: Uuid,
        title: String,
        completed: bool,
    ) -> Result<todo::Model, DbError>;
    async fn delete(&self, id: Uuid) -> Result<(), DbError>;
}

#[derive(Clone)]
pub struct TodoRepositoryImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl TodoRepository for TodoRepositoryImpl {
    async fn get_by_id(&self, id: Uuid) -> Result<Model, DbError> {
        TodoEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map(|opt| opt.ok_or(DbError::NotFound))?
    }

    async fn create(&self, title: String) -> Result<Model, DbError> {
        let date_time_now = DateTime::from(Utc::now());

        let new_todo = todo::ActiveModel {
            id: Set(Uuid::new_v4()),
            title: Set(title),
            completed: Set(false),
            updated_at: Set(date_time_now),
            created_at: Set(date_time_now),
        };

        let res = TodoEntity::insert(new_todo).exec(&self.db).await?;
        TodoEntity::find_by_id(res.last_insert_id)
            .one(&self.db)
            .await
            .map(|opt| opt.ok_or(DbError::NotFound))?
    }

    async fn update(&self, id: Uuid, title: String, completed: bool) -> Result<Model, DbError> {
        let todo_found_result = TodoEntity::find_by_id(id).one(&self.db).await?;
        let original_todo = todo_found_result.ok_or(DbError::NotFound)?;

        let updated_todo = todo::ActiveModel {
            id: Set(id),
            title: Set(title),
            completed: Set(completed),
            created_at: Set(original_todo.created_at),
            updated_at: Set(DateTime::from(Utc::now())),
        };

        Ok(TodoEntity::update(updated_todo).exec(&self.db).await?);
    }

    async fn delete(&self, id: Uuid) -> Result<(), DbError> {
        let todo = TodoEntity::find_by_id(id).one(&self.db).await?;
        todo.ok_or(DbError::NotFound)?;

        TodoEntity::delete_by_id(id).exec(&self.db).await?;

        Ok(())
    }
}
