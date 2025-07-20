use chrono::{DateTime, Utc};
use entity::todo::{ActiveModel, Entity, Model};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set, TryIntoModel};
use uuid::Uuid;

use async_trait::async_trait;

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> Result<Model, DbErr>;
    async fn create(&self, title: String) -> Result<Model, DbErr>;
    async fn update(&self, id: Uuid, title: String, completed: bool) -> Result<Model, DbErr>;
    async fn delete(&self, id: Uuid) -> Result<(), DbErr>;
}

#[derive(Clone)]
pub struct TodoRepositoryImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl TodoRepository for TodoRepositoryImpl {
    async fn get_by_id(&self, id: Uuid) -> Result<Model, DbErr> {
        Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(DbErr::RecordNotFound("Todo not found".to_string()))
    }

    async fn create(&self, title: String) -> Result<Model, DbErr> {
        let new_todo = ActiveModel {
            title: Set(title),
            ..Default::default()
        }
        .save(&self.db)
        .await?;

        Ok(new_todo.try_into_model()?)
    }

    async fn update(&self, id: Uuid, title: String, completed: bool) -> Result<Model, DbErr> {
        let updated_todo = ActiveModel {
            id: Set(id),
            title: Set(title),
            completed: Set(completed),
            updated_at: Set(DateTime::from(Utc::now())),
            ..Default::default()
        }
        .update(&self.db)
        .await?;

        Ok(updated_todo.try_into_model()?)
    }

    async fn delete(&self, id: Uuid) -> Result<(), DbErr> {
        Entity::delete_by_id(id).exec(&self.db).await?;

        Ok(())
    }
}
