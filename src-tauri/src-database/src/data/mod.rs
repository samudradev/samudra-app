//! Exposed representations of data from database.
//!
//! Five data is available for external consumption:
//! 1. Lemma
//! 2. Konsep
//! 3. GolonganKata
//! 4. Cakupan
//! 5. KataAsing

#[macro_use]
mod export;

use itertools::Itertools;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::cakupan::Model as Cakupan;
use crate::models::golongan_kata::Model as GolonganKata;
use crate::models::kata_asing::Model as KataAsing;
use crate::models::konsep::Model as Konsep;
use crate::models::lemma::Model as Lemma;
use crate::query::LemmaSQLUnit;

#[derive(Debug, Serialize, Deserialize, PartialEq, diff::Diff)]
pub struct LemmaDataRepr {
    pub lemma: String,
    pub konseps: Vec<KonsepDataRepr>,
}

#[allow(dead_code)]
impl LemmaDataRepr {
    pub(crate) fn from(value: Vec<LemmaSQLUnit>) -> Vec<LemmaDataRepr> {
        let mut data = Vec::<LemmaDataRepr>::new();
        let m = value
            .clone()
            .into_iter()
            .into_group_map_by(|a| a.lemma.clone())
            .into_iter()
            .map(|(k, v): (String, Vec<LemmaSQLUnit>)| {
                (
                    k,
                    v.into_iter()
                        .into_group_map_by(|a| a.konsep.as_ref().unwrap().clone()),
                )
            })
            .collect::<HashMap<String, HashMap<String, Vec<LemmaSQLUnit>>>>();
        for (k, hm) in m.iter() {
            let mut kvec = Vec::<KonsepDataRepr>::new();
            for (khm, hmhm) in hm.iter() {
                kvec.push(KonsepDataRepr {
                    keterangan: khm.clone(),
                    golongan_kata: hmhm[0].golongan_kata.clone().unwrap().clone(),
                    cakupans: hmhm.clone().into_iter().fold(vec![], |mut a, b| {
                        a.push(b.cakupan.clone().unwrap());
                        a
                    }),
                    kata_asing: hmhm.clone().into_iter().fold(vec![], |mut a, b| {
                        a.push(KataAsingRepr {
                            nama: b.kata_asing.clone().unwrap(),
                            bahasa: b.bahasa_asing.clone().unwrap(),
                        });
                        a
                    }),
                })
            }
            data.push(LemmaDataRepr {
                lemma: k.clone(),
                konseps: kvec,
            })
        }
        data
    }
}
#[derive(Debug, Serialize, Deserialize, PartialEq, diff::Diff)]
pub struct KonsepDataRepr {
    pub keterangan: String,
    pub golongan_kata: String,
    pub cakupans: Vec<String>,
    pub kata_asing: Vec<KataAsingRepr>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, diff::Diff)]
pub struct KataAsingRepr {
    pub nama: String,
    pub bahasa: String,
}

export! {
    LemmaData from Lemma with {
        id: i32,
        nama: String;
        attachment {
            konseps: Konsep => ..KonsepData
        }
    }
}

export! {
    KonsepData from Konsep with {
        id: i32;
        rename golongan_id to golongan_kata: GolonganKataData;
        optional {
            keterangan: String,
            tertib: i32
        };
        attachment {
            cakupan: Cakupan => ..CakupanData,
            kata_asing: KataAsing => ..KataAsingData
        }
    }
}

export! {
    CakupanData from Cakupan with {
        id: i32,
        nama: String;
        optional {
            keterangan: String
        }
    }
}

export! {
    KataAsingData from KataAsing with {
        id: i32,
        nama: String,
        bahasa: String
    }
}
export! {
    GolonganKataData from GolonganKata with {
        id: String;
        optional {
            nama: String,
            keterangan: String
        }
    }
}

impl From<String> for GolonganKataData {
    fn from(value: String) -> Self {
        GolonganKataData {
            id: value,
            ..Default::default()
        }
    }
}
