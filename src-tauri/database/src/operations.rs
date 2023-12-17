use async_trait::async_trait;
use diff::{Diff, VecDiff, VecDiffType};
use konsep::KonsepItemDiff;

use crate::data::items::{kata_asing, konsep};
use crate::data::{CakupanItem, CakupanItemDiff, KataAsingItemDiff, LemmaItem, LemmaItemDiff};
use crate::insertions::ToTableWithReference;
use crate::prelude::{KataAsingItem, KonsepItem};
use crate::types::DbProvidedDiff;

#[async_trait]
pub trait DiffSumbittable<DB: sqlx::Database>: diff::Diff {
    async fn submit_changes(self, new: &Self, pool: &sqlx::Pool<DB>) -> sqlx::Result<()>;
}

async fn match_cakupan_changes(
    diffs: &VecDiffType<CakupanItem>,
    konsep_old: &KonsepItem,
    pool: &sqlx::Pool<sqlx::Sqlite>,
) {
    match diffs {
        VecDiffType::Inserted { index, changes } => {
            for ch in changes {
                CakupanItem::null()
                    .apply_new(&ch)
                    .insert_safe_with_reference(konsep_old, pool)
                    .await
                    .expect("Error");
            }
        }
        VecDiffType::Altered { index, changes } => {
            todo!("Cakupan Altered: {:#?}", changes)
        }
        VecDiffType::Removed { index, len } => {
            for i in *index..*index + *len {
                konsep_old.cakupans[i]
                    .clone()
                    .detach_from(konsep_old, pool)
                    .await
                    .expect("Error");
            }
        }
    }
}
async fn match_kata_asing_changes(
    diffs: &VecDiffType<KataAsingItem>,
    konsep_old: &KonsepItem,
    pool: &sqlx::Pool<sqlx::Sqlite>,
) {
    match diffs {
        VecDiffType::Inserted { index, changes } => {
            for ch in changes {
                KataAsingItem::null()
                    .apply_new(&ch)
                    .insert_safe_with_reference(konsep_old, pool)
                    .await
                    .expect("Error");
            }
        }
        VecDiffType::Altered { index, changes } => {
            todo!("KataAsing Altered: {:#?}", changes)
        }
        VecDiffType::Removed { index, len } => {
            for i in *index..*index + *len {
                konsep_old.kata_asing[i]
                    .clone()
                    .detach_from(konsep_old, pool)
                    .await
                    .expect("Error");
            }
        }
    }
}

#[async_trait]
impl DiffSumbittable<sqlx::Sqlite> for LemmaItem {
    async fn submit_changes(self, new: &Self, pool: &sqlx::Pool<sqlx::Sqlite>) -> sqlx::Result<()> {
        let diff = dbg!(self.clone().diff(new));
        match diff {
            LemmaItemDiff {
                konseps,
                lemma: None,                  // No change in lemma
                id: DbProvidedDiff::NoChange, // No change in lemma id
            } => {
                if konseps.0.len() == 0 {
                    // Since diff found no difference, old.konseps and new.konseps is assumed the same
                    // and zipped together to find diff in childrens: golongan_kata, cakupan, kata_asing
                    for (konsep_old, konsep_new) in self.konseps.iter().zip(new.konseps.clone()) {
                        for diffs in konsep_old.cakupans.diff(&konsep_new.cakupans).0 {
                            let _ = match_cakupan_changes(&diffs, &konsep_old, pool).await;
                        }
                        for diffs in konsep_old.kata_asing.diff(&konsep_new.kata_asing).0 {
                            let _ = match_kata_asing_changes(&diffs, &konsep_old, pool).await;
                        }
                    }
                } else {
                    for (kondiff, konsep_old) in konseps.0.iter().zip(self.konseps.clone()) {
                        match kondiff {
                            VecDiffType::Altered { index, changes } => {
                                for alter_konsep in changes {
                                    match alter_konsep.keterangan {
                                        None => {
                                            let cakupans = &alter_konsep.cakupans.0;
                                            let kata_asing = &alter_konsep.kata_asing.0;
                                            for diffs in cakupans {
                                                let _ =
                                                    match_cakupan_changes(diffs, &konsep_old, pool)
                                                        .await;
                                            }
                                            for diffs in kata_asing {
                                                let _ = match_kata_asing_changes(
                                                    diffs,
                                                    &konsep_old,
                                                    pool,
                                                )
                                                .await;
                                            }
                                        }
                                        Some(_) => {
                                            todo!("Change keterangan")
                                        }
                                    }
                                }
                            }
                            VecDiffType::Inserted { index, changes } => {
                                for insert_konsep in changes {
                                    KonsepItem::null()
                                        .apply_new(&insert_konsep)
                                        .insert_safe_with_reference(&self, pool)
                                        .await
                                        .expect("Insertion update fail");
                                }
                            }
                            VecDiffType::Removed { index, len } => todo!("{:#?}", &kondiff),
                        }
                    }
                }
            }
            _ => todo!("{:#?}", diff),
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::data::items::lemma::Item;
    use crate::data::{KataAsingItem, KonsepItem, LemmaItem};
    use crate::operations::DiffSumbittable;
    use crate::prelude::{LemmaWithKonsepView, ToTable};
    use crate::types::DbProvided;
    use sqlx::{Pool, Sqlite};

    #[sqlx::test(fixtures("lemma"))]
    fn test_diff_handling(pool: Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let view = LemmaWithKonsepView::query_all(&pool).await?;
        let data = LemmaItem::from_views(&view);
        let old = data
            .first()
            .expect("Vec<LemmaDataRepr> is zero sized")
            .to_owned();
        assert_eq!(&old.konseps.len(), &1);
        let new: LemmaItem = LemmaItem {
            id: DbProvided::Known(1),
            lemma: "cakera tokokan".into(),
            konseps: vec![
                KonsepItem {
                    id: DbProvided::Known(1),
                    keterangan: "gas-gas dan debu yang mengelilingi lohong hitam".into(),
                    golongan_kata: "kata nama".into(),
                    cakupans: vec!["Astrofizik".into(), "Teori Relativiti".into()],
                    kata_asing: vec![KataAsingItem {
                        nama: "accretion disk".into(),
                        bahasa: "english".into(),
                    }],
                },
                KonsepItem {
                    id: DbProvided::Unknown,
                    keterangan: "konsep baharu yang tiada kena mengena".into(),
                    golongan_kata: "kata nama".into(),
                    cakupans: vec![],
                    kata_asing: vec![],
                },
            ],
        };
        old.submit_changes(&new, &pool).await?;
        let view = LemmaWithKonsepView::query_all(&pool).await?;
        let data = LemmaItem::from_views(&view);
        assert_eq!(data, vec![new]);
        Ok(())
    }
    #[sqlx::test(fixtures("lemma"))]
    fn test_diff_handling_detach_cakupan(pool: Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let view = LemmaWithKonsepView::query_all(&pool).await?;
        let data = LemmaItem::from_views(&view);
        let old = data
            .first()
            .expect("Vec<LemmaDataRepr> is zero sized")
            .to_owned();
        assert_eq!(&old.konseps.len(), &1);
        let new: LemmaItem = LemmaItem {
            id: DbProvided::Known(1),
            lemma: "cakera tokokan".into(),
            konseps: vec![KonsepItem {
                id: DbProvided::Known(1),
                keterangan: "gas-gas dan debu yang mengelilingi lohong hitam".into(),
                golongan_kata: "kata nama".into(),
                cakupans: vec!["Astrofizik".into()],
                kata_asing: vec![KataAsingItem {
                    nama: "accretion disk".into(),
                    bahasa: "english".into(),
                }],
            }],
        };
        old.submit_changes(&new, &pool).await?;
        let view = LemmaWithKonsepView::query_all(&pool).await?;
        let data = LemmaItem::from_views(&view);
        assert_eq!(
            data.first()
                .expect("Here?")
                .konseps
                .first()
                .expect("Konsep")
                .cakupans,
            vec!["Astrofizik".into()]
        );
        Ok(())
    }
    #[sqlx::test(fixtures("lemma"))]
    fn test_diff_handling_detach_kata_asing(pool: Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let view = LemmaWithKonsepView::query_all(&pool).await?;
        let data = LemmaItem::from_views(&view);
        let old = data
            .first()
            .expect("Vec<LemmaDataRepr> is zero sized")
            .to_owned();
        assert_eq!(&old.konseps.len(), &1);
        let new: LemmaItem = LemmaItem {
            id: DbProvided::Known(1),
            lemma: "cakera tokokan".into(),
            konseps: vec![KonsepItem {
                id: DbProvided::Known(1),
                keterangan: "gas-gas dan debu yang mengelilingi lohong hitam".into(),
                golongan_kata: "kata nama".into(),
                cakupans: vec!["Astrofizik".into(), "Teori Relativiti".into()],
                kata_asing: vec![],
            }],
        };
        old.submit_changes(&new, &pool).await?;
        let view = LemmaWithKonsepView::query_all(&pool).await?;
        let data = LemmaItem::from_views(&view);
        assert_eq!(
            data.first()
                .expect("Here?")
                .konseps
                .first()
                .expect("Konsep")
                .kata_asing,
            vec![]
        );
        Ok(())
    }

    #[sqlx::test(fixtures("defaults"))]
    fn test_insert_safe(pool: Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let new: LemmaItem = LemmaItem {
            id: DbProvided::Unknown,
            lemma: "cakera tokokan".into(),
            konseps: vec![KonsepItem {
                id: DbProvided::Unknown,
                keterangan: "gas-gas dan debu yang mengelilingi lohong hitam".into(),
                golongan_kata: "kata nama".into(),
                cakupans: vec!["Astrofizik".into(), "Teori Relativiti".into()],
                kata_asing: vec![KataAsingItem {
                    nama: "accretion disk".into(),
                    bahasa: "english".into(),
                }],
            }],
        };
        let _insert = new.clone().insert_safe(&pool).await?;
        let view = LemmaWithKonsepView::query_all(&pool).await?;
        let data = LemmaItem::from_views(&view);
        let from_db = data.first().expect("Lemma Item?");
        assert_eq!(from_db, &new);
        Ok(())
    }
}
