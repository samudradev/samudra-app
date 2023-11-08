//! A crate to handle database operations.

pub mod data;
pub mod io;
mod models;
pub mod query;

use data::{LemmaData, LemmaDataDiff};
pub use sea_orm::DatabaseConnection;

use sea_orm::prelude::async_trait::async_trait;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ActiveValue, ConnectionTrait, Database, DbErr,
    EntityTrait, IntoActiveModel, TryIntoModel,
};
use serde::{Deserialize, Serialize};
use sqlx::error::Error;
use sqlx::migrate::MigrateDatabase;
use sqlx::{Sqlite, SqlitePool};

use diff::{Diff, OptionDiff, VecDiffType};

use crate::models::konsep;

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
trait CheckDuplicateTrait<E>:
    ActiveModelTrait<Entity = E> + ActiveModelBehavior + TryIntoModel<E::Model> + Sync + Send
where
    E: EntityTrait,
{
    async fn check<C>(self, db: &C) -> Result<Self, DbErr>
    where
        C: ConnectionTrait;

    async fn insert_with_check<C>(
        &self,
        key: <E as EntityTrait>::Column,
        db: &C,
    ) -> Result<E::Model, DbErr>
    where
        <E as EntityTrait>::Model: IntoActiveModel<Self>,
        C: ConnectionTrait,
    {
        match self.clone().check(db).await {
            Err(_e) => todo!(),
            Ok(am) => match am.get(key) {
                ActiveValue::Unchanged(_) => Ok(am.try_into_model()?),
                ActiveValue::NotSet => Ok(am.insert(db).await?),
                _ => todo!(),
            },
        }
    }
}

#[allow(unused_variables)]
pub async fn handle_changes(
    old: &LemmaData,
    new: &LemmaData,
    db: &DatabaseConnection,
) -> Result<(), String> {
    let diff = old.diff(new);
    // dbg!(&diff);
    match diff {
        // Same ID
        LemmaDataDiff {
            id: 0,
            konseps: OptionDiff::Some(v),
            ..
        } => {
            for kon in v.0.iter() {
                match kon {
                    // TODO Implement changes
                    VecDiffType::Inserted { index, changes } => {
                        for k in changes.iter() {
                            if let OptionDiff::Some(ket) = &k.keterangan {
                                // TODO GET GOLONGAN KATA DATA
                                let g = if &k.golongan_kata.id == &Some(String::from("undefined")) {
                                    Some(String::from("NAMA"))
                                } else {
                                    k.golongan_kata.id.clone()
                                };
                                konsep::ActiveModel {
                                    lemma_id: ActiveValue::Set(old.id),
                                    golongan_id: ActiveValue::Set(g.clone()),
                                    keterangan: ActiveValue::Set(ket.clone()),
                                    ..Default::default()
                                }
                                .insert(db)
                                .await
                                .unwrap();
                            }
                        }
                    }
                    VecDiffType::Altered { index, changes } => todo!("Altered konsep"),
                    VecDiffType::Removed { index, len } => todo!("Removed konsep"),
                }
            }
            Ok(())
        }
        LemmaDataDiff { .. } => {
            todo!("Literally anything else: {:#?}", &diff);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::data::{KataAsingRepr, KonsepDataRepr, LemmaDataRepr};
    use crate::query::LemmaSQLUnit;
    use crate::{data::LemmaData, handle_changes};
    use sea_orm::SqlxSqliteConnector;
    use sqlx::{Pool, Sqlite};

    // TODO implement handle_changes for LemmaDataRepr
    #[sqlx::test(fixtures("lemma"))]
    fn test_diff_handling(pool: Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let unit = LemmaSQLUnit::query_all(&pool).await.unwrap();
        let data = LemmaDataRepr::from(unit);
        assert_eq!(data.first().unwrap().konseps.len(), 1);
        let _old = data.first().unwrap().to_owned();
        let jsonstr = r#"{
            "id": 1,
            "nama": "cakera tokokan",
            "konseps": vec![
                {   
                    "id": 1,
                    "golongan_kata": {"id": "NAMA"},
                    "keterangan": "gas-gas dan debu yang mengelilingi lohong hitam"
                },
                {
                    "id": 2,
                    "golongan_kata": {"id": "NAMA"},
                    "keterangan": "data baharu yang tiada kena mengena"
                }
                ]
            }"#;
        let _new: LemmaData = serde_json::from_str(jsonstr).unwrap();
        // let _db = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool);
        // TODO
        // handle_changes(old, &new, &db).await.unwrap();
        let v = LemmaDataRepr::from(LemmaSQLUnit::query_all(&pool).await.unwrap());
        assert_eq!(v.first().unwrap().konseps.len(), 2);
        Ok(())
    }

    #[sqlx::test(fixtures("lemma", "lemma_2"))]
    fn test_custom_sql(pool: Pool<Sqlite>) {
        let unit = LemmaSQLUnit::query_all(&pool).await.unwrap();
        let mut data = LemmaDataRepr::from(unit);
        assert_eq!(
            data.sort_by(|a, b| a.lemma.cmp(&b.lemma)),
            vec![
                LemmaDataRepr {
                    lemma: "ufuk peristiwa".into(),
                    konseps: vec![KonsepDataRepr {
                        keterangan: "sempadan terluar lohong hitam".into(),
                        golongan_kata: "NAMA".into(),
                        cakupans: vec!["Astrofizik".into(), "Teori Relativiti".into(),],
                        kata_asing: vec![
                            KataAsingRepr {
                                nama: "event horizon".into(),
                                bahasa: "english".into(),
                            },
                            KataAsingRepr {
                                nama: "event horizon".into(),
                                bahasa: "english".into(),
                            },
                        ],
                    },],
                },
                LemmaDataRepr {
                    lemma: "cakera tokokan".into(),
                    konseps: vec![KonsepDataRepr {
                        keterangan: "gas-gas dan debu yang mengelilingi lohong hitam".into(),
                        golongan_kata: "NAMA".into(),
                        cakupans: vec!["Astrofizik".into(), "Teori Relativiti".into(),],
                        kata_asing: vec![
                            KataAsingRepr {
                                nama: "accretion disk".into(),
                                bahasa: "english".into(),
                            },
                            KataAsingRepr {
                                nama: "accretion disk".into(),
                                bahasa: "english".into(),
                            },
                        ],
                    },],
                },
            ]
            .sort_by(|a, b| a.lemma.cmp(&b.lemma))
        );
    }
}
