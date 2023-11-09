use sea_orm::Database;
pub use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use sqlx::migrate::MigrateDatabase;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DatabaseConfig {
    pub path: String,
    pub engine: DatabaseEngine,
}

impl DatabaseConfig {
    pub fn full_url(&self) -> String {
        match &self.engine {
            DatabaseEngine::SQLite => format!("sqlite:{}", self.path),
        }
    }

    pub async fn connect(&self) -> DatabaseConnection {
        match self.engine {
            DatabaseEngine::SQLite => Database::connect(format!("sqlite:{}", self.path))
                .await
                .unwrap_or(DatabaseConnection::Disconnected),
        }
    }

    pub async fn create_and_migrate(&self) -> sqlx::Result<()> {
        match &self.engine {
            DatabaseEngine::SQLite => {
                let url = self.full_url();
                sqlx::Sqlite::create_database(&url).await?;
                let pool = sqlx::SqlitePool::connect(&url).await?;
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
