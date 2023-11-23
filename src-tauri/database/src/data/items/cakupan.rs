use crate::data::KonsepItem;
use crate::errors::{BackendError, Result};
use crate::insertions::ReferenceItem;
use crate::insertions::{ToTable, ToTableWithReference};
use crate::models::cakupan::{Cakupan, InsertCakupan};
use crate::models::cakupan_x_konsep::CakupanXKonsep;
use crate::models::JointTable;

use ormlite::model::Insertable;
use ormlite::Model;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, diff::Diff)]
#[diff(attr(
    #[derive(Debug)]
))]
pub struct CakupanItem(String);

impl From<&str> for CakupanItem {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}
impl From<String> for CakupanItem {
    fn from(value: String) -> Self {
        Self(value)
    }
}
#[async_trait::async_trait]
impl ToTable<sqlx::Sqlite> for CakupanItem {
    type OUTPUT = Cakupan;

    async fn insert_safe(self, pool: &sqlx::Pool<sqlx::Sqlite>) -> Result<Cakupan> {
        match Cakupan::select()
            .where_bind("nama = ?", &self.0)
            .fetch_optional(pool)
            .await?
        {
            Some(c) => Ok(c),
            None => InsertCakupan {
                nama: self.0.clone(),
                keterangan: None,
            }
            .insert(pool)
            .await
            .map_err(BackendError::from),
        }
    }
}

#[async_trait::async_trait]
impl ToTableWithReference<sqlx::Sqlite> for CakupanItem {
    type OUTPUT = Cakupan;
    type REFERENCE = KonsepItem;

    async fn insert_safe_with_reference(
        self,
        reference: &Self::REFERENCE,
        pool: &sqlx::Pool<sqlx::Sqlite>,
    ) -> Result<Cakupan> {
        let cakupan = <Self as ToTable<sqlx::Sqlite>>::insert_safe(self, pool).await?;
        let _ = CakupanXKonsep {
            cakupan_id: cakupan.id,
            konsep_id: reference.reference_field(),
        }
        .insert_safe(pool)
        .await
        .map_err(BackendError::from);
        Ok(cakupan)
    }
}
