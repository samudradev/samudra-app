use crate::prelude::*;

use crate::data::items::cakupan::CakupanItem;
use crate::data::items::lemma::LemmaItem;

use crate::models::golongan_kata::GolonganKata;
use crate::models::konsep::{InsertKonsep, Konsep};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diff::Diff, ts_rs::TS)]
#[ts(export, export_to = "../../src/bindings/")]
#[diff(attr(
    #[derive(Debug)]
))]
pub struct KonsepItem {
    pub id: DbProvided<i32>,
    pub keterangan: String,
    pub golongan_kata: String,
    #[ts(type = "Array<string>")]
    pub cakupans: Vec<CakupanItem>,
    pub kata_asing: Vec<KataAsingItem>,
}

impl PartialEq for KonsepItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.keterangan == other.keterangan
            && self.golongan_kata == other.golongan_kata
            // Necessary to ignore vector order
            && other
                .cakupans
                .iter()
                .filter(|&a| !self.cakupans.contains(a))
                .collect_vec()
                .len()
                == 0
            && other
                .kata_asing
                .iter()
                .filter(|&a| !self.kata_asing.contains(a))
                .collect_vec()
                .len()
                == 0
    }
}
impl ReferenceItem for KonsepItem {
    type FIELD = i32;
    fn reference_field(&self) -> Self::FIELD {
        match self.id {
            DbProvided::Known(v) => v,
            DbProvided::Unknown => todo!(),
        }
    }
}
#[async_trait::async_trait]
impl ToTableWithReference<sqlx::Sqlite> for KonsepItem {
    type OUTPUT = Konsep;
    type REFERENCE = LemmaItem;

    #[allow(unreachable_code)]
    async fn insert_safe_with_reference(
        self,
        reference: &Self::REFERENCE,
        pool: &sqlx::Pool<sqlx::Sqlite>,
    ) -> Result<Konsep> {
        let konsep = match self.id {
            DbProvided::Known(v) => {
                let q = Konsep::select().where_bind("id = ?", v);
                q.fetch_one(pool).await?
            }
            DbProvided::Unknown => match Konsep::select()
                .where_bind("keterangan = ?", self.keterangan.clone())
                .fetch_optional(pool)
                .await?
            {
                Some(l) => l,
                None => {
                    let golkat = GolonganKata::select()
                        .where_bind("id = ?", &self.golongan_kata)
                        .fetch_one(pool)
                        .await
                        .expect(&format!(
                            "A row from 'golongan_kata' where id = '{}' is expected",
                            self.golongan_kata
                        ));
                    InsertKonsep {
                        lemma_id: reference.reference_field(),
                        golongan_id: golkat.id,
                        keterangan: Some(self.keterangan.clone()),
                        tertib: None,
                    }
                    .insert(pool)
                    .await?
                }
            },
        };
        let konsep_item = KonsepItem {
            id: DbProvided::Known(konsep.id),
            ..self.clone()
        };
        for cakupan in self.clone().cakupans.into_iter().clone() {
            cakupan
                .insert_safe_with_reference(&konsep_item, pool)
                .await?;
        }
        for kata_asing in self.clone().kata_asing.into_iter().clone() {
            kata_asing
                .insert_safe_with_reference(&konsep_item, pool)
                .await?;
        }

        Ok(konsep)
    }
}
