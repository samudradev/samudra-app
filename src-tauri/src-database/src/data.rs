//! Exposed representations of data from database.

use async_trait::async_trait;
use itertools::Itertools;
use ormlite::model::Insertable;
use ormlite::Model;
use std::{collections::HashMap, fmt::Debug};

use serde::{Deserialize, Serialize};

use crate::{
    errors::{BackendError, Result},
    insertions::{ReferenceItem, ToTable, ToTableWithReference},
    models::{
        cakupan::{Cakupan, InsertCakupan},
        cakupan_x_konsep::CakupanXKonsep,
        golongan_kata::GolonganKata,
        kata_asing::{InsertKataAsing, KataAsing},
        kata_asing_x_konsep::KataAsingXKonsep,
        konsep::{InsertKonsep, Konsep},
        lemma::{InsertLemma, Lemma},
        JointTable,
    },
    types::DbProvided,
    views::LemmaWithKonsepView,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, diff::Diff, ts_rs::TS)]
#[ts(export, export_to = "../../src/bindings/")]
#[diff(attr(
    #[derive(Debug)]
))]
pub struct LemmaItem {
    pub id: DbProvided<i32>,
    pub lemma: String,
    pub konseps: Vec<KonsepItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, diff::Diff, ts_rs::TS)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, diff::Diff)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, diff::Diff, ts_rs::TS)]
#[ts(export, export_to = "../../src/bindings/")]
#[diff(attr(
    #[derive(Debug)]
))]
pub struct KataAsingItem {
    pub nama: String,
    pub bahasa: String,
}

impl ReferenceItem for LemmaItem {
    type FIELD = i32;
    fn reference_field(&self) -> Self::FIELD {
        match self.id {
            DbProvided::Known(v) => v,
            DbProvided::Unknown => todo!(),
        }
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

#[async_trait]
impl ToTable<sqlx::Sqlite> for LemmaItem {
    type OUTPUT = Lemma;
    async fn insert_safe(self, pool: &sqlx::Pool<sqlx::Sqlite>) -> Result<Lemma> {
        let lemma = match self.id {
            DbProvided::Known(v) => {
                Lemma::select()
                    .where_bind("id = ?", v)
                    .fetch_one(pool)
                    .await?
            }
            DbProvided::Unknown => match Lemma::select()
                .where_bind("nama = ?", self.lemma.clone())
                .fetch_optional(pool)
                .await?
            {
                Some(l) => l,
                None => {
                    InsertLemma {
                        nama: self.lemma.clone(),
                    }
                    .insert(pool)
                    .await?
                }
            },
        };
        let lemma_item = LemmaItem {
            id: DbProvided::Known(lemma.id),
            ..self.clone()
        };
        for kon in self.clone().konseps.into_iter().clone() {
            kon.insert_safe_with_reference(&lemma_item, pool).await?;
        }
        Ok(lemma)
    }
}

#[async_trait]
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

#[async_trait]
impl ToTableWithReference<sqlx::Sqlite> for CakupanItem {
    type OUTPUT = Cakupan;
    type REFERENCE = KonsepItem;

    async fn insert_safe_with_reference(
        self,
        reference: &Self::REFERENCE,
        pool: &sqlx::Pool<sqlx::Sqlite>,
    ) -> Result<Cakupan> {
        let cakupan = <Self as ToTable<sqlx::Sqlite>>::insert_safe(self, pool).await?;
        CakupanXKonsep {
            cakupan_id: cakupan.id,
            konsep_id: reference.reference_field(),
        }
        .insert_safe(pool)
        .await
        .map_err(BackendError::from);
        Ok(cakupan)
    }
}
#[async_trait]
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

#[async_trait]
impl ToTableWithReference<sqlx::Sqlite> for KataAsingItem {
    type OUTPUT = KataAsing;
    type REFERENCE = KonsepItem;

    async fn insert_safe_with_reference(
        self,
        reference: &Self::REFERENCE,
        pool: &sqlx::Pool<sqlx::Sqlite>,
    ) -> Result<KataAsing> {
        let kata_asing = <Self as ToTable<sqlx::Sqlite>>::insert_safe(self, pool).await?;
        KataAsingXKonsep {
            konsep_id: reference.reference_field(),
            kata_asing_id: kata_asing.id,
        }
        .insert_safe(pool)
        .await
        .map_err(BackendError::from);
        Ok(kata_asing)
    }
}

#[async_trait]
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
                todo!("So apparently this part throws `RowNotFound` error. Investigate.");
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
                        .await?;
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

impl LemmaItem {
    pub fn from_views(views: Vec<LemmaWithKonsepView>) -> Vec<LemmaItem> {
        let mut data = Vec::<LemmaItem>::new();
        let lemma_map: HashMap<(i32, String), HashMap<(i32, String, String), Vec<LemmaWithKonsepView>>> = views
            .clone()
            .into_iter()
            .into_group_map_by(|a| (a.l_id, a.lemma.clone()))
            .into_iter()
            .map(|(k, v): ((i32, String), Vec<LemmaWithKonsepView>)| {
                (
                    k,
                    v.into_iter()
                        .into_group_map_by(|a| (a.k_id, a.konsep.as_ref().unwrap().clone(), a.golongan_kata.clone().unwrap_or_default())),
                )
            })
            .collect::<HashMap<(i32, String), HashMap<(i32, String, String), Vec<LemmaWithKonsepView>>>>();
        for (lemma, konsep_map) in lemma_map.iter() {
            let mut kvec = Vec::<KonsepItem>::new();
            for (konsep, views) in konsep_map.iter() {
                kvec.push(KonsepItem {
                    id: DbProvided::Known(konsep.0.clone()),
                    keterangan: konsep.1.clone(),
                    golongan_kata: konsep.2.clone(),
                    cakupans: views.clone().into_iter().fold(vec![], |mut a, b| {
                        if let Some(c) = b.cakupan.clone() {
                            a.push(c.into());
                        }
                        a
                    }),
                    kata_asing: views.clone().into_iter().fold(vec![], |mut a, b| {
                        if let (Some(nama), Some(bahasa)) =
                            (b.kata_asing.clone(), b.bahasa_asing.clone())
                        {
                            a.push(KataAsingItem { nama, bahasa });
                        }
                        a
                    }),
                })
            }
            data.push(LemmaItem {
                id: DbProvided::Known(lemma.0.clone()),
                lemma: lemma.1.clone(),
                konseps: kvec,
            })
        }
        data
    }
}
