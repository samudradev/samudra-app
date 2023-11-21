use crate::data::items::konsep::KonsepItem;
use crate::errors::{BackendError, Result};
use crate::insertions::ReferenceItem;
use crate::insertions::{ToTable, ToTableWithReference};
use crate::models::kata_asing::{InsertKataAsing, KataAsing};
use crate::models::kata_asing_x_konsep::KataAsingXKonsep;
use crate::models::JointTable;

use ormlite::model::Insertable;
use ormlite::Model;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, diff::Diff, ts_rs::TS)]
#[ts(export, export_to = "../../src/bindings/")]
#[diff(attr(
    #[derive(Debug)]
))]
pub struct KataAsingItem {
    pub nama: String,
    pub bahasa: String,
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
        let _ = KataAsingXKonsep {
            konsep_id: reference.reference_field(),
            kata_asing_id: kata_asing.id,
        }
        .insert_safe(pool)
        .await
        .map_err(BackendError::from);
        Ok(kata_asing)
    }
}
