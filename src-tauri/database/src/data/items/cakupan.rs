use crate::data::Item;
use crate::prelude::*;

use crate::models::cakupan::{Cakupan, InsertCakupan};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, diff::Diff, Hash, Eq)]
#[diff(attr(
    #[derive(Debug)]
))]
pub struct CakupanItem(String);

impl CakupanItem {
    pub fn null() -> Self {
        CakupanItem("".into())
    }
}

impl Item for CakupanItem {
    type MAP = (); //PLACEHOLDER;

    type VIEW = LemmaWithKonsepView;

    fn from_hashmap(_value: &Self::MAP) -> Vec<Self> {
        todo!()
    }

    fn from_views(value: &Vec<Self::VIEW>) -> Vec<Self> {
        value
            .clone()
            .into_iter()
            .filter(|a| a.cakupan.is_some())
            .into_group_map_by(|a| a.cakupan.clone().expect("None have been filtered out"))
            .keys()
            .map(|a| Self::from(a.to_owned()))
            .collect_vec()
    }
}

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
    type OUTPUT = ();
    type REFERENCE = KonsepItem;

    async fn insert_safe_with_reference(
        self,
        reference: &Self::REFERENCE,
        pool: &sqlx::Pool<sqlx::Sqlite>,
    ) -> Result<()> {
        let refer = reference.reference_field();
        sqlx::query! {"INSERT or IGNORE INTO cakupan (nama) VALUES (?)", self.0}
            .execute(pool)
            .await
            .expect("Error inserting cakupan");
        sqlx::query! {"INSERT or IGNORE INTO cakupan_x_konsep (cakupan_id, konsep_id) VALUES ((SELECT id FROM cakupan WHERE cakupan.nama = ?), ?)",
        self.0,
        refer
    }
    .execute(pool)
    .await
    .expect("Error inserting cakupan_x_konsep");
        Ok(())
    }
    async fn detach_from(
        self,
        reference: &Self::REFERENCE,
        pool: &sqlx::Pool<sqlx::Sqlite>,
    ) -> Result<()> {
        let refer = reference.reference_field();
        sqlx::query! {
            "DELETE FROM cakupan_x_konsep AS cxk WHERE cxk.cakupan_id = (SELECT id FROM  cakupan WHERE cakupan.nama = ?) AND cxk.konsep_id = ?",
            self.0,
            refer
        }
        .execute(pool)
        .await
        .expect("Error detaching cakupan from konsep");
        Ok(())
    }
}
