use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, Database, Sqlite};

#[derive(Debug, Clone)]
pub struct ActiveDatabase {
    pub pool: sqlx::Pool<Sqlite>,
    pub path: String,
}

impl ActiveDatabase {
    pub async fn renew(&mut self, value: DatabaseConfig) -> sqlx::Result<()> {
        self.pool = sqlx::SqlitePool::connect(&value.path).await?;
        self.path = value.path.clone().into();
        Ok(())
    }

    pub async fn from(value: DatabaseConfig) -> Self {
        Self {
            pool: sqlx::SqlitePool::connect(&value.path).await.unwrap(),
            path: value.path.clone().into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DatabaseConfig {
    pub path: String,
}

impl DatabaseConfig {
    pub async fn create_and_migrate(&self) -> sqlx::Result<()> {
        sqlx::Sqlite::create_database(&self.path).await?;
        let pool = sqlx::SqlitePool::connect(&self.path).await?;
        sqlx::migrate!().run(&pool).await?;
        Ok(())
    }
}
