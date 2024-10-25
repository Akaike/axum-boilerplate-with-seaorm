use axum::async_trait;
use sqlx::{PgPool, Postgres, Transaction};

#[async_trait]
pub trait DatabaseManager: Send + Sync {
    fn get_pool(&self) -> &PgPool;
    async fn start_transaction(&self) -> sqlx::Result<Transaction<'_, Postgres>>;
    async fn commit_transaction(&self, transaction: Transaction<'_, Postgres>) -> sqlx::Result<()>;
    async fn rollback_transaction(
        &self,
        transaction: Transaction<'_, Postgres>,
    ) -> sqlx::Result<()>;
}

pub struct DatabaseManagerImpl {
    pool: PgPool,
}

impl DatabaseManagerImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DatabaseManager for DatabaseManagerImpl {
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }

    async fn start_transaction(&self) -> sqlx::Result<Transaction<'_, Postgres>> {
        self.pool.begin().await
    }

    async fn commit_transaction(&self, transaction: Transaction<'_, Postgres>) -> sqlx::Result<()> {
        transaction.commit().await
    }

    async fn rollback_transaction(
        &self,
        transaction: Transaction<'_, Postgres>,
    ) -> sqlx::Result<()> {
        transaction.rollback().await
    }
}
