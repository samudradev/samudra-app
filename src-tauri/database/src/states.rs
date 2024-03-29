//! Connections to database

use sqlx::migrate::MigrateDatabase;

use crate::errors::Result;
use crate::prelude::BackendError;

pub use sqlx::Pool;
pub use sqlx::Sqlite;

// TODO: Refactor this module

/// Connection to database
#[derive(Debug, Clone)]
pub struct Connection {
    #[allow(missing_docs)]
    pub pool: Pool<Sqlite>,
}

/// Counts of selected items.
#[allow(missing_docs)]
#[derive(Debug, Clone, serde::Serialize, PartialEq, ts_rs::TS)]
#[ts(export, export_to = "../../src/bindings/")]
pub struct Counts {
    lemmas: i32,
    konseps: i32,
    golongan_katas: i32,
    cakupans: i32,
    kata_asings: i32,
}

/// A helper struct for when a single string value is queried.
pub struct StringItem {
    #[allow(missing_docs)]
    pub item: String,
}

impl Connection {
    /// Forcefully reconnect to the specified url.
    pub async fn renew(mut self, url: String) -> Result<Self> {
        self.pool = sqlx::SqlitePool::connect(&url).await?;
        Ok(self)
    }

    /// Acquires a connection to the specified url.
    ///
    /// If exists, applies the migration to current version.
    /// If not exist, applies [Self::create_and_migrate] to the same url.
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

    /// Queries all `golongan_kata.nama` and returns Result<Vec<[StringItem]>>
    ///
    /// The function calls the following query:
    /// ```sql
    /// SELECT nama AS item FROM golongan_kata
    /// ```
    pub async fn get_golongan_kata_enumeration(self) -> Result<Vec<StringItem>> {
        sqlx::query_as!(StringItem, "SELECT nama AS item FROM golongan_kata")
            .fetch_all(&self.pool)
            .await
            .map_err(BackendError::from)
    }

    /// Queries the counts of each items as [Counts].
    ///
    /// The function calls the following query:
    /// ```sql
    #[doc = include_str!("../transactions/count_items.sql")]
    /// ```
    pub async fn statistics(self) -> Result<Counts> {
        sqlx::query_file_as!(Counts, "transactions/count_items.sql")
            .fetch_one(&self.pool)
            .await
            .map_err(BackendError::from)
    }

    /// Creates the Sqlite database in the specified url then apply migrations.
    pub async fn create_and_migrate(url: String) -> Result<Self> {
        sqlx::Sqlite::create_database(&url).await?;
        let pool = sqlx::SqlitePool::connect(&url).await?;
        sqlx::migrate!().run(&pool).await?;
        Ok(Self { pool })
    }

    /// Populates the database with presets to satisfy certain constrains.
    ///
    /// The current constrain is that the `konsep.golongan_kata` field must point to a `golongan_kata.id`.
    /// This function calls the following query:
    /// ```sql
    #[doc = include_str!("../presets/golongan_kata_ms-my.sql")]
    /// ```
    pub async fn populate_with_presets(self) -> Result<Self> {
        sqlx::query_file!("presets/golongan_kata_ms-my.sql")
            .execute(&self.pool)
            .await?;
        Ok(self)
    }
}
