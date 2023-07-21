//! A crate to handle database operations.

pub mod data;
mod models;
pub mod query;

pub use sea_orm::DatabaseConnection;

use sea_orm::Database;
use serde::{Deserialize, Serialize};
use sqlx::error::Error;
use sqlx::migrate::MigrateDatabase;
use sqlx::{Sqlite, SqlitePool};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DatabaseConfig {
    pub path: String,
    pub engine: DatabaseEngine,
}

impl DatabaseConfig {
    pub async fn connect(&self) -> DatabaseConnection {
        match self.engine {
            DatabaseEngine::SQLite => Database::connect(format!("sqlite:{}", self.path))
                .await
                .unwrap_or(DatabaseConnection::Disconnected),
        }
    }

    pub async fn create_and_migrate(&self) -> Result<(), Error> {
        match &self.engine {
            DatabaseEngine::SQLite => {
                let url = format!("sqlite:{}", self.path);
                Sqlite::create_database(&url).await?;
                let pool = SqlitePool::connect(&url).await?;
                sqlx::migrate!().run(&pool).await?;
                Ok(())
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "lowercase")]
pub enum DatabaseEngine {
    #[default]
    SQLite,
}
