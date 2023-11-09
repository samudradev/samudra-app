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

use diff::{Diff, OptionDiff};
use itertools::Itertools;
use sea_orm::ActiveModelTrait;
use sqlx::Sqlite;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{models::konsep, views::LemmaWithKonsepView};

// use crate::models::cakupan::Model as Cakupan;
// use crate::models::golongan_kata::Model as GolonganKata;
// use crate::models::kata_asing::Model as KataAsing;
// use crate::models::konsep::Model as Konsep;
// use crate::models::lemma::Model as Lemma;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, diff::Diff)]
pub struct LemmaDataRepr {
    pub id: i32,
    pub lemma: String,
    pub konseps: Vec<KonsepDataRepr>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, diff::Diff)]
pub struct KonsepDataRepr {
    pub id: i32,
    pub keterangan: String,
    pub golongan_kata: String,
    pub cakupans: Vec<String>,
    pub kata_asing: Vec<KataAsingRepr>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, diff::Diff)]
pub struct KataAsingRepr {
    pub nama: String,
    pub bahasa: String,
}

impl LemmaDataRepr {
    pub fn from_views(views: Vec<LemmaWithKonsepView>) -> Vec<LemmaDataRepr> {
        let mut data = Vec::<LemmaDataRepr>::new();
        let lemma_map: HashMap<(i32, String), HashMap<(i32, String), Vec<LemmaWithKonsepView>>> = views
            .clone()
            .into_iter()
            .into_group_map_by(|a| (a.l_id, a.lemma.clone()))
            .into_iter()
            .map(|(k, v): ((i32, String), Vec<LemmaWithKonsepView>)| {
                (
                    k,
                    v.into_iter()
                        .into_group_map_by(|a| (a.k_id, a.konsep.as_ref().unwrap().clone())),
                )
            })
            .collect::<HashMap<(i32, String), HashMap<(i32, String), Vec<LemmaWithKonsepView>>>>();
        for (lemma, konsep_map) in lemma_map.iter() {
            let mut kvec = Vec::<KonsepDataRepr>::new();
            for (konsep, views) in konsep_map.iter() {
                kvec.push(KonsepDataRepr {
                    id: konsep.0.clone(),
                    keterangan: konsep.1.clone(),
                    golongan_kata: views[0].golongan_kata.clone().unwrap().clone(),
                    cakupans: views.clone().into_iter().fold(vec![], |mut a, b| {
                        if let Some(c) = b.cakupan.clone() {
                            a.push(c);
                        }
                        a
                    }),
                    kata_asing: views.clone().into_iter().fold(vec![], |mut a, b| {
                        if let (Some(nama), Some(bahasa)) =
                            (b.kata_asing.clone(), b.bahasa_asing.clone())
                        {
                            a.push(KataAsingRepr { nama, bahasa });
                        }
                        a
                    }),
                })
            }
            data.push(LemmaDataRepr {
                id: lemma.0.clone(),
                lemma: lemma.1.clone(),
                konseps: kvec,
            })
        }
        data
    }
}

// export! {
//     LemmaData from Lemma with {
//         id: i32,
//         nama: String;
//         attachment {
//             konseps: Konsep => ..KonsepData
//         }
//     }
// }

// export! {
//     KonsepData from Konsep with {
//         id: i32;
//         rename golongan_id to golongan_kata: GolonganKataData;
//         optional {
//             keterangan: String,
//             tertib: i32
//         };
//         attachment {
//             cakupan: Cakupan => ..CakupanData,
//             kata_asing: KataAsing => ..KataAsingData
//         }
//     }
// }

// export! {
//     CakupanData from Cakupan with {
//         id: i32,
//         nama: String;
//         optional {
//             keterangan: String
//         }
//     }
// }

// export! {
//     KataAsingData from KataAsing with {
//         id: i32,
//         nama: String,
//         bahasa: String
//     }
// }
// export! {
//     GolonganKataData from GolonganKata with {
//         id: String;
//         optional {
//             nama: String,
//             keterangan: String
//         }
//     }
// }

// impl From<String> for GolonganKataData {
//     fn from(value: String) -> Self {
//         GolonganKataData {
//             id: value,
//             ..Default::default()
//         }
//     }
// }
