//! A crate to handle database operations.

pub mod data;
pub mod io;
mod models;
pub mod query;

pub use sea_orm::DatabaseConnection;

use sea_orm::{ActiveModelBehavior, ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, Database, DbErr, EntityTrait, IntoActiveModel, ModelTrait, TryIntoModel};
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::sea_query::ValueTuple;
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
    pub fn full_url(&self) -> String {
        match &self.engine {
            DatabaseEngine::SQLite => format!("sqlite:{}", self.path),
        }
    }

    pub async fn count_lemma(&self) -> sqlx::Result<()> {
        let pool = SqlitePool::connect(&self.full_url()).await?;

        let res = sqlx::query!("SELECT count(lemma.id) as count from lemma")
            .fetch_one(&pool)
            .await;
        dbg!(res?);
        Ok(())
    }

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
                let url = self.full_url();
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

#[async_trait]
trait CheckDuplicateTrait<E>: ActiveModelTrait<Entity=E> + ActiveModelBehavior + TryIntoModel<E::Model> + Sync + Send where E: EntityTrait {

    async fn check<C>(self, db: &C) -> Result<Self, DbErr> where C: ConnectionTrait;

    async fn insert_with_check<C>(&self, key: <E as EntityTrait>::Column, db: &C) -> Result<E::Model, DbErr>
        where  <E as EntityTrait>::Model: IntoActiveModel<Self>, C: ConnectionTrait {
        match self.clone().check(db).await {
            Err(e) => todo!(),
            Ok(am) => {
                match am.get(key) {
                    ActiveValue::Unchanged(_) => Ok(am.try_into_model()?),
                    ActiveValue::NotSet => Ok(am.insert(db).await?),
                    _ => todo!()
                }
            }
        }
    }
}