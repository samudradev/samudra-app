use sqlx::migrate::MigrateDatabase;

use crate::errors::Result;
use crate::prelude::BackendError;

pub use sqlx::Pool;
pub use sqlx::Sqlite;

#[derive(Debug, Clone)]
pub struct Connection {
    pub pool: Pool<Sqlite>,
}

#[derive(Debug, Clone, serde::Serialize, PartialEq, ts_rs::TS)]
#[ts(export, export_to = "../../src/bindings/")]
pub struct Counts {
    lemmas: i32,
    konseps: i32,
    golongan_katas: i32,
    cakupans: i32,
    kata_asings: i32,
}

pub struct StringItem {
    pub item: String,
}

impl Connection {
    pub async fn renew(mut self, url: String) -> Result<Self> {
        self.pool = sqlx::SqlitePool::connect(&url).await?;
        Ok(self)
    }

    pub async fn from(url: String) -> Self {
        match sqlx::SqlitePool::connect(&url).await {
            Ok(pool) => {
                // Automatically migrate to current version
                sqlx::migrate!().run(&pool).await.expect("Migration error");
                Self { pool }
            }
            Err(_) => dbg!(Self::create_and_migrate(url)
                .await
                .unwrap()
                .populate_with_presets()
                .await
                .unwrap()),
        }
    }

    pub async fn get_golongan_kata_enumeration(self) -> Result<Vec<StringItem>> {
        sqlx::query_as!(StringItem, "SELECT nama AS item FROM golongan_kata")
            .fetch_all(&self.pool)
            .await
            .map_err(BackendError::from)
    }

    pub async fn statistics(self) -> Result<Counts> {
        sqlx::query_file_as!(Counts, "transactions/count_items.sql")
            .fetch_one(&self.pool)
            .await
            .map_err(BackendError::from)
    }

    pub async fn create_and_migrate(url: String) -> Result<Self> {
        sqlx::Sqlite::create_database(&url).await?;
        let pool = sqlx::SqlitePool::connect(&url).await?;
        sqlx::migrate!().run(&pool).await?;
        Ok(Self { pool })
    }

    pub async fn populate_with_presets(self) -> Result<Self> {
        sqlx::query_file!("presets/golongan_kata_ms-my.sql")
            .execute(&self.pool)
            .await?;
        Ok(self)
    }
}
