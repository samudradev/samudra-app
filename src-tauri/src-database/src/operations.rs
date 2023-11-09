#![allow(unused_variables)]

use diff::{Diff, OptionDiff, VecDiffType};
use sea_orm::{
    prelude::async_trait::async_trait, ActiveModelTrait, ActiveValue, DatabaseConnection,
    SqlxSqliteConnector,
};

use crate::{
    data::{KonsepDataRepr, LemmaDataRepr, LemmaDataReprDiff},
    models::konsep,
};

#[async_trait]
trait DiffSumbittable<DB: sqlx::Database>: diff::Diff {
    async fn submit_changes(self, new: &Self, db: &sqlx::Pool<DB>) -> sqlx::Result<()>;
}

#[async_trait]
impl DiffSumbittable<sqlx::Sqlite> for LemmaDataRepr {
    async fn submit_changes(self, new: &Self, db: &sqlx::Pool<sqlx::Sqlite>) -> sqlx::Result<()> {
        let diff = self.clone().diff(new);
        match diff {
            LemmaDataReprDiff {
                konseps,
                // lemma: None, // No change in lemma
                // id: 0,       // No change in lemma id
                ..
            } => {
                for kon in konseps.0.into_iter() {
                    match kon {
                        diff::VecDiffType::Inserted { index: _, changes } => {
                            for c in changes.into_iter() {
                                let new = konsep::ActiveModel {
                                    id: sea_orm::ActiveValue::NotSet,
                                    tarikh_masuk: sea_orm::ActiveValue::NotSet,
                                    lemma_id: sea_orm::ActiveValue::Set(self.id),
                                    golongan_id: sea_orm::ActiveValue::Set(c.golongan_kata),
                                    keterangan: sea_orm::ActiveValue::Set(c.keterangan),
                                    tertib: sea_orm::ActiveValue::NotSet,
                                };
                                new.insert(&SqlxSqliteConnector::from_sqlx_sqlite_pool(db.clone()))
                                    .await
                                    .unwrap();
                                // TODO append tags
                            }
                        }
                        diff::VecDiffType::Removed { index: _, len } => todo!("Konsep Removed!"),
                        diff::VecDiffType::Altered { index: _, changes } => {
                            todo!("Konsep Altered!")
                        }
                    }
                }
            }
        };
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::data::{KataAsingRepr, KonsepDataRepr, LemmaDataRepr};
    use crate::operations::DiffSumbittable;
    use crate::query::QueryView;
    use sqlx::{Pool, Sqlite};

    // TODO implement handle_changes for LemmaDataRepr
    #[sqlx::test(fixtures("lemma"))]
    fn test_diff_handling(pool: Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let view = QueryView::new().all(&pool).await?;
        let data = LemmaDataRepr::from_views(view);
        let _old = data
            .first()
            .expect("Vec<LemmaDataRepr> is zero sized")
            .to_owned();
        assert_eq!(&_old.konseps.len(), &1);
        let _new: LemmaDataRepr = LemmaDataRepr {
            id: 1,
            lemma: "cakera tokokan".into(),
            konseps: vec![
                KonsepDataRepr {
                    id: 1,
                    keterangan: "gas-gas dan debu yang mengelilingi lohong hitam".into(),
                    golongan_kata: "NAMA".into(),
                    cakupans: vec!["Astrofizik".into(), "Teori Relativiti".into()],
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
                },
                KonsepDataRepr {
                    id: 2,
                    keterangan: "konsep baharu yang tiada kena mengena".into(),
                    golongan_kata: "NAMA".into(),
                    cakupans: vec![],
                    kata_asing: vec![],
                },
            ],
        };
        _old.submit_changes(&_new, &pool).await?;
        let view = QueryView::new().all(&pool).await?;
        let data = LemmaDataRepr::from_views(view);
        assert_eq!(data.first().expect("Here?").konseps.len(), 2);
        Ok(())
    }
}
