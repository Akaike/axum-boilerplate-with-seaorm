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
        let new_todo = todo::ActiveModel {
            id: Set(Uuid::new_v4()),
            title: Set(title),
            completed: Set(false),
        };

        let res = TodoEntity::insert(new_todo).exec(&self.db).await?;
        TodoEntity::find_by_id(res.last_insert_id)
            .one(&self.db)
            .await
            .map(|opt| opt.ok_or(DbError::NotFound))?
    }

    async fn update(&self, id: Uuid, title: String, completed: bool) -> Result<Model, DbError> {
        let todo = TodoEntity::find_by_id(id).one(&self.db).await?;
        todo.ok_or(DbError::NotFound)?;

        let updated_todo = todo::ActiveModel {
            id: Set(id),
            title: Set(title),
            completed: Set(completed),
        };

        TodoEntity::update(updated_todo).exec(&self.db).await?;
        TodoEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map(|opt| opt.ok_or(DbError::NotFound))?
    }
}
