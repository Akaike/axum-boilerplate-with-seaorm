use chrono::{DateTime, Utc};
use entity::todo::{ActiveModel, Entity, Model};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, TryIntoModel};
use uuid::Uuid;

use async_trait::async_trait;

use crate::common::error::DatabaseError;

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> Result<Model, DatabaseError>;
    async fn create(&self, title: String) -> Result<Model, DatabaseError>;
    async fn update(&self, id: Uuid, title: String, completed: bool) -> Result<Model, DatabaseError>;
    async fn delete(&self, id: Uuid) -> Result<(), DatabaseError>;
}

#[derive(Clone)]
pub struct TodoRepositoryImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl TodoRepository for TodoRepositoryImpl {
    async fn get_by_id(&self, id: Uuid) -> Result<Model, DatabaseError> {
        Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(DatabaseError::NotFound)
    }

    async fn create(&self, title: String) -> Result<Model, DatabaseError> {
        let new_todo = ActiveModel {
            title: Set(title),
            ..Default::default()
        }.save(&self.db).await?;

        Ok(new_todo.try_into_model()?)
    }

    async fn update(&self, id: Uuid, title: String, completed: bool) -> Result<Model, DatabaseError> {
        let updated_todo = ActiveModel {
            id: Set(id),
            title: Set(title),
            completed: Set(completed),
            updated_at: Set(DateTime::from(Utc::now())),
            ..Default::default()
        }.update(&self.db).await?;

        Ok(updated_todo.try_into_model()?)
    }

    async fn delete(&self, id: Uuid) -> Result<(), DatabaseError> {
        Entity::delete_by_id(id).exec(&self.db).await?;

        Ok(())
    }
}
