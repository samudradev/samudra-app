use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, Sqlite};

use crate::errors::Result;

#[derive(Debug, Clone)]
pub struct Connection {
    pub pool: sqlx::Pool<Sqlite>,
}

impl Connection {
    pub async fn renew(mut self, url: String) -> Result<Self> {
        self.pool = sqlx::SqlitePool::connect(&url).await?;
        Ok(self)
    }

    pub async fn from(url: String) -> Self {
        Self {
            pool: sqlx::SqlitePool::connect(&url).await.unwrap(),
        }
    }

    pub async fn create_and_migrate(url: String) -> Result<Self> {
        sqlx::Sqlite::create_database(&url).await?;
        let pool = sqlx::SqlitePool::connect(&url).await?;
        sqlx::migrate!().run(&pool).await?;
        Ok(Self { pool })
    }
}
