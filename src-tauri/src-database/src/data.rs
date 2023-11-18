//! Exposed representations of data from database.
//!
//! Five data is available for external consumption:
//! 1. Lemma
//! 2. Konsep
//! 3. GolonganKata
//! 4. Cakupan
//! 5. KataAsing

// #[macro_use]
// mod export;

use async_trait::async_trait;
use itertools::Itertools;
use ormlite::{model::Insertable, Model, Result, TableMeta};
use sqlx::Sqlite;
use std::{collections::HashMap, fmt::Debug};
use ts_rs::TS;

use serde::{Deserialize, Serialize};

use crate::{
    models::{
        cakupan::{Cakupan, InsertCakupan},
        cakupan_x_konsep::CakupanXKonsep,
        golongan_kata::GolonganKata,
        kata_asing::{InsertKataAsing, KataAsing},
        kata_asing_x_konsep::KataAsingXKonsep,
        konsep::{InsertKonsep, Konsep},
        lemma::{InsertLemma, Lemma},
    },
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

// struct AuxillaryItem<L, R>(L, R)
// where
//     L: TS + Debug + PartialEq,
//     R: TS + Debug + PartialEq;

// #[async_trait]
// impl ToTable<sqlx::Sqlite> for AuxillaryItem<i32, i32> {
//     type INSERT = CakupanXKonsep;

//     async fn insert_safe(
//         self,
//         pool: &sqlx::Pool<sqlx::Sqlite>,
//     ) -> Result<<Self::INSERT as Insertable<sqlx::Sqlite>>::Model> {
//         CakupanXKonsep::insert(self, conn)
//     }
// }
// #[async_trait]
// impl ToTable<sqlx::Sqlite> for AuxillaryItem<i32, i32> {
//     type INSERT = KataAsingXKonsep;

//     async fn insert_safe(
//         self,
//         pool: &sqlx::Pool<sqlx::Sqlite>,
//     ) -> Result<<Self::INSERT as Insertable<sqlx::Sqlite>>::Model> {
//         todo!()
//     }
// }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, diff::Diff)]
#[diff(attr(
    #[derive(Debug)]
))]
pub enum DbProvided<T>
where
    T: TS + Debug + PartialEq + diff::Diff,
    <T as diff::Diff>::Repr: Debug,
{
    Known(T),
    Unknown,
}

/// IdentInDB<T> acts like an option in JS/TS
impl<T> TS for DbProvided<T>
where
    T: TS + Debug + PartialEq + diff::Diff,
    <T as diff::Diff>::Repr: Debug,
{
    fn name() -> String {
        <Option<T> as TS>::name()
    }

    fn dependencies() -> Vec<ts_rs::Dependency> {
        <Option<T> as TS>::dependencies()
    }

    fn transparent() -> bool {
        <Option<T> as TS>::transparent()
    }

    fn name_with_type_args(args: Vec<String>) -> String {
        <Option<T> as TS>::name_with_type_args(args)
    }

    fn inline() -> String {
        <Option<T> as TS>::inline()
    }

    fn inline_flattened() -> String {
        <Option<T> as TS>::inline_flattened()
    }
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

pub trait ReferenceItem {
    type FIELD;
    fn reference_field(&self) -> Self::FIELD;
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
pub trait ToTable<DB: sqlx::Database> {
    type INSERT: Insertable<DB> + Sized + Send + Sync;

    /// Insert values while checking for duplicates
    async fn insert_safe(
        self,
        pool: &sqlx::Pool<DB>,
    ) -> Result<<Self::INSERT as Insertable<DB>>::Model>;
}
#[async_trait]
pub trait ToTableWithReference<DB: sqlx::Database> {
    type INSERT: Insertable<DB> + Sized + Send + Sync;
    type REFERENCE: ReferenceItem;

    /// Insert values while checking for duplicates with referred value from `reference`.
    async fn insert_safe_with_reference(
        self,
        reference: &Self::REFERENCE,
        pool: &sqlx::Pool<DB>,
    ) -> Result<<Self::INSERT as Insertable<DB>>::Model>;
}

#[async_trait]
impl ToTable<sqlx::Sqlite> for LemmaItem {
    type INSERT = InsertLemma;
    async fn insert_safe(
        self,
        pool: &sqlx::Pool<Sqlite>,
    ) -> Result<<Self::INSERT as Insertable<sqlx::Sqlite>>::Model> {
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
    type INSERT = InsertCakupan;

    async fn insert_safe(
        self,
        pool: &sqlx::Pool<sqlx::Sqlite>,
    ) -> Result<<Self::INSERT as Insertable<sqlx::Sqlite>>::Model> {
        match Cakupan::select()
            .where_bind("nama = ?", &self.0)
            .fetch_optional(pool)
            .await?
        {
            Some(c) => Ok(c),
            None => {
                InsertCakupan {
                    nama: self.0.clone(),
                    keterangan: None,
                }
                .insert(pool)
                .await
            }
        }
    }
}

#[async_trait]
impl ToTableWithReference<sqlx::Sqlite> for CakupanItem {
    type INSERT = InsertCakupan;
    type REFERENCE = KonsepItem;

    async fn insert_safe_with_reference(
        self,
        reference: &Self::REFERENCE,
        pool: &sqlx::Pool<sqlx::Sqlite>,
    ) -> Result<<Self::INSERT as Insertable<sqlx::Sqlite>>::Model> {
        let cakupan = <Self as ToTable<sqlx::Sqlite>>::insert_safe(self, pool).await?;
        CakupanXKonsep {
            cakupan_id: cakupan.id,
            konsep_id: reference.reference_field(),
        }
        .insert_safe(pool)?;
        Ok(cakupan)
    }
}
#[async_trait]
impl ToTable<sqlx::Sqlite> for KataAsingItem {
    type INSERT = InsertKataAsing;

    async fn insert_safe(
        self,
        pool: &sqlx::Pool<sqlx::Sqlite>,
    ) -> Result<<Self::INSERT as Insertable<sqlx::Sqlite>>::Model> {
        match KataAsing::select()
            .where_("nama = ? AND bahasa = ?")
            .bind(&self.nama)
            .bind(&self.bahasa)
            .fetch_optional(pool)
            .await?
        {
            Some(c) => Ok(c),
            None => {
                InsertKataAsing {
                    nama: self.nama.clone(),
                    bahasa: self.bahasa.clone(),
                }
                .insert(pool)
                .await
            }
        }
    }
}

#[async_trait]
impl ToTableWithReference<sqlx::Sqlite> for KataAsingItem {
    type INSERT = InsertKataAsing;
    type REFERENCE = KonsepItem;

    async fn insert_safe_with_reference(
        self,
        reference: &Self::REFERENCE,
        pool: &sqlx::Pool<sqlx::Sqlite>,
    ) -> Result<<Self::INSERT as Insertable<sqlx::Sqlite>>::Model> {
        let kata_asing = <Self as ToTable<sqlx::Sqlite>>::insert_safe(self, pool).await?;
        KataAsingXKonsep {
            konsep_id: reference.reference_field(),
            kata_asing_id: kata_asing.id,
        }
        .insert_safe(pool)?;
        Ok(kata_asing)
    }
}

// use ormlite

#[async_trait]
impl ToTableWithReference<sqlx::Sqlite> for KonsepItem {
    type INSERT = InsertKonsep;
    type REFERENCE = LemmaItem;

    async fn insert_safe_with_reference(
        self,
        reference: &Self::REFERENCE,
        pool: &sqlx::Pool<Sqlite>,
    ) -> Result<<Self::INSERT as Insertable<sqlx::Sqlite>>::Model> {
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
                    Self::INSERT {
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
