use entity::todo::{self, Entity as TodoEntity, Model};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, Set};
use uuid::Uuid;

use async_trait::async_trait;

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> Result<todo::Model, sea_orm::DbErr>;
    async fn create(&self, title: String) -> Result<todo::Model, sea_orm::DbErr>;
}

#[derive(Clone)]
pub struct TodoRepositoryImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl TodoRepository for TodoRepositoryImpl {
    async fn get_by_id(&self, id: Uuid) -> Result<Model, DbErr> {
        TodoEntity::find_by_id(id).one(&self.db).await.map(|opt| {
            opt.ok_or(DbErr::RecordNotFound(format!(
                "Todo with id {} not found",
                id
            )))
        })?
    }

    async fn create(&self, title: String) -> Result<Model, DbErr> {
        let new_todo = todo::ActiveModel {
            id: Set(Uuid::new_v4()),
            title: Set(title),
            completed: Set(false),
        };

        let res = TodoEntity::insert(new_todo).exec(&self.db).await?;
        TodoEntity::find_by_id(res.last_insert_id)
            .one(&self.db)
            .await
            .map(|opt| {
                opt.ok_or(DbErr::Custom(format!(
                    "Failed to retrieve created todo with id {}",
                    res.last_insert_id
                )))
            })?
    }
}
