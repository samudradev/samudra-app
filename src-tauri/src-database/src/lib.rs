//! A crate to handle database operations.

pub mod data;
mod models;
pub mod query;

pub use sea_orm::DatabaseConnection;

use sea_orm::Database;
use serde::{Deserialize, Serialize};

// TODO Create database file

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
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "lowercase")]
pub enum DatabaseEngine {
    #[default]
    SQLite,
}
