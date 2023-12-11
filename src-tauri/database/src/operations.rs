use async_trait::async_trait;
use diff::Diff;

use crate::data::{LemmaItem, LemmaItemDiff};
use crate::insertions::ToTableWithReference;

#[async_trait]
trait DiffSumbittable<DB: sqlx::Database>: diff::Diff {
    async fn submit_changes(self, new: &Self, pool: &sqlx::Pool<DB>) -> sqlx::Result<()>;
}

#[async_trait]
impl DiffSumbittable<sqlx::Sqlite> for LemmaItem {
    async fn submit_changes(self, new: &Self, pool: &sqlx::Pool<sqlx::Sqlite>) -> sqlx::Result<()> {
        let diff = self.clone().diff(new);
        match diff {
            LemmaItemDiff {
                konseps,
                // lemma: None, // No change in lemma
                // id: 0,       // No change in lemma id
                ..
            } => {
                for kon in self.konseps.apply_new(&konseps) {
                    kon.insert_safe_with_reference(&self, pool)
                        .await
                        .expect("Row not found");
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::data::items::lemma::Item;
    use crate::data::{KataAsingItem, KonsepItem, LemmaItem};
    use crate::operations::DiffSumbittable;
    use crate::prelude::ToTable;
    use crate::query::QueryView;
    use crate::types::DbProvided;
    use sqlx::{Pool, Sqlite};

    #[sqlx::test(fixtures("lemma"))]
    fn test_diff_handling(pool: Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let view = QueryView::new().all(&pool).await?;
        let data = LemmaItem::from_views(&view);
        let _old = data
            .first()
            .expect("Vec<LemmaDataRepr> is zero sized")
            .to_owned();
        assert_eq!(&_old.konseps.len(), &1);
        let _new: LemmaItem = LemmaItem {
            id: DbProvided::Known(1),
            lemma: "cakera tokokan".into(),
            konseps: vec![
                KonsepItem {
                    id: DbProvided::Unknown,
                    keterangan: "gas-gas dan debu yang mengelilingi lohong hitam".into(),
                    golongan_kata: "NAMA".into(),
                    cakupans: vec!["Astrofizik".into(), "Teori Relativiti".into()],
                    kata_asing: vec![
                        KataAsingItem {
                            nama: "accretion disk".into(),
                            bahasa: "english".into(),
                        },
                        KataAsingItem {
                            nama: "accretion disk".into(),
                            bahasa: "english".into(),
                        },
                    ],
                },
                KonsepItem {
                    id: DbProvided::Unknown,
                    keterangan: "konsep baharu yang tiada kena mengena".into(),
                    golongan_kata: "NAMA".into(),
                    cakupans: vec![],
                    kata_asing: vec![],
                },
            ],
        };
        _old.submit_changes(&_new, &pool).await?;
        let view = QueryView::new().all(&pool).await?;
        let data = LemmaItem::from_views(&view);
        assert_eq!(data.first().expect("Here?").konseps.len(), 2);
        Ok(())
    }

    #[sqlx::test(fixtures("defaults"))]
    fn test_insert_safe(pool: Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let new: LemmaItem = LemmaItem {
            id: DbProvided::Unknown,
            lemma: "cakera tokokan".into(),
            konseps: vec![
                KonsepItem {
                    id: DbProvided::Unknown,
                    keterangan: "gas-gas dan debu yang mengelilingi lohong hitam".into(),
                    golongan_kata: "NAMA".into(),
                    cakupans: vec!["Astrofizik".into(), "Teori Relativiti".into()],
                    kata_asing: vec![
                        KataAsingItem {
                            nama: "accretion disk".into(),
                            bahasa: "english".into(),
                        },
                        KataAsingItem {
                            nama: "accretion disk".into(),
                            bahasa: "english".into(),
                        },
                    ],
                },
                KonsepItem {
                    id: DbProvided::Unknown,
                    keterangan: "konsep baharu yang tiada kena mengena".into(),
                    golongan_kata: "NAMA".into(),
                    cakupans: vec![],
                    kata_asing: vec![],
                },
            ],
        };
        let _insert = dbg!(new.clone().insert_safe(&pool).await?);
        let view = QueryView::new().all(&pool).await?;
        let data = LemmaItem::from_views(&view);
        let from_db = data.first().expect("Lemma Item?");
        assert_eq!(&from_db.lemma, &new.lemma);
        assert_eq!(
            &from_db.konseps.first().expect("Konsep item?").keterangan,
            &new.konseps.first().expect("Konsep item?").keterangan
        );
        assert_eq!(
            &from_db.konseps.last().expect("Konsep item?").keterangan,
            &new.konseps.last().expect("Konsep item?").keterangan
        );
        Ok(())
    }
}
