//! Exposed representations of data from database.
//!
//! Five data is available for external consumption:
//! 1. Lemma
//! 2. Konsep
//! 3. GolonganKata
//! 4. Cakupan
//! 5. KataAsing

use serde::{Deserialize, Serialize};

use crate::models::{konsep, lemma};

#[derive(Debug, Serialize, Deserialize)]
pub struct LemmaData {
    pub id: i32,
    pub nama: String,
    pub konseps: Option<Vec<KonsepData>>,
}

impl From<lemma::Model> for LemmaData {
    fn from(value: lemma::Model) -> Self {
        LemmaData {
            id: value.id,
            nama: value.nama,
            konseps: None,
        }
    }
}

impl From<(lemma::Model, Vec<konsep::Model>)> for LemmaData {
    fn from(value: (lemma::Model, Vec<konsep::Model>)) -> Self {
        // check uniqueness (panic if theres two different lemmas)
        let mut lem = LemmaData::from(value.0.clone());
        lem.konseps = Some(
            value
                .1
                .into_iter()
                .map(|kon| KonsepData::from(kon))
                .collect::<Vec<KonsepData>>(),
        );
        lem
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KonsepData {
    pub id: i32,
    pub golongan_kata: GolonganKataData,
    pub keterangan: Option<String>,
    pub tertib: Option<i32>,
    pub cakupan: Option<Vec<CakupanData>>,
    pub kata_asing: Option<Vec<KataAsingData>>,
}

impl From<konsep::Model> for KonsepData {
    fn from(value: konsep::Model) -> Self {
        KonsepData {
            id: value.id,
            golongan_kata: GolonganKataData {
                id: value.golongan_id.unwrap(),
                ..Default::default()
            },
            keterangan: value.keterangan,
            tertib: value.tertib,
            cakupan: None,
            kata_asing: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GolonganKataData {
    pub id: String,
    pub nama: String,
    pub keterangan: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CakupanData {
    pub id: i32,
    pub nama: String,
    pub keterangan: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KataAsingData {
    pub id: i32,
    pub nama: String,
    pub bahasa: String,
}
