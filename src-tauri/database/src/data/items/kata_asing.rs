use crate::data::Item;
use crate::prelude::*;

use crate::models::kata_asing::{InsertKataAsing, KataAsing};
use crate::models::kata_asing_x_konsep::KataAsingXKonsep;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, diff::Diff, ts_rs::TS)]
#[ts(export, export_to = "../../src/bindings/")]
#[diff(attr(
    #[derive(Debug)]
))]
pub struct KataAsingItem {
    pub nama: String,
    pub bahasa: String,
}

impl Item for KataAsingItem {
    type MAP = (); //PLACEHOLDER

    type VIEW = LemmaWithKonsepView;

    fn from_hashmap(value: &Self::MAP) -> Vec<Self> {
        todo!()
    }

    fn from_views(value: &Vec<Self::VIEW>) -> Vec<Self> {
        value
            .clone()
            .into_iter()
            .filter(|a| a.kata_asing.is_some())
            .into_group_map_by(|a| {
                (
                    a.kata_asing
                        .clone()
                        .expect("None should have been filtered out"),
                    a.bahasa_asing
                        .clone()
                        .expect("None should have been filtered out"),
                )
            })
            .keys()
            .map(|(nama, bahasa)| KataAsingItem {
                nama: nama.into(),
                bahasa: bahasa.into(),
            })
            .collect_vec()
    }
}

#[async_trait::async_trait]
impl ToTable<sqlx::Sqlite> for KataAsingItem {
    type OUTPUT = KataAsing;

    async fn insert_safe(self, pool: &sqlx::Pool<sqlx::Sqlite>) -> Result<KataAsing> {
        match KataAsing::select()
            .where_("nama = ? AND bahasa = ?")
            .bind(&self.nama)
            .bind(&self.bahasa)
            .fetch_optional(pool)
            .await?
        {
            Some(c) => Ok(c),
            None => InsertKataAsing {
                nama: self.nama.clone(),
                bahasa: self.bahasa.clone(),
            }
            .insert(pool)
            .await
            .map_err(BackendError::from),
        }
    }
}

#[async_trait::async_trait]
impl ToTableWithReference<sqlx::Sqlite> for KataAsingItem {
    type OUTPUT = KataAsing;
    type REFERENCE = KonsepItem;

    async fn insert_safe_with_reference(
        self,
        reference: &Self::REFERENCE,
        pool: &sqlx::Pool<sqlx::Sqlite>,
    ) -> Result<KataAsing> {
        let kata_asing = <Self as ToTable<sqlx::Sqlite>>::insert_safe(self, pool).await?;
        let refer = reference.reference_field();
        sqlx::query! {"INSERT or IGNORE INTO kata_asing_x_konsep (kata_asing_id, konsep_id) VALUES (?, ?)",
            kata_asing.id,
            refer
        }
        .execute(pool)
        .await
        .expect("Error inserting cakupan_x_konsep");
        Ok(kata_asing)
    }
}
