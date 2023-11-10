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

use itertools::Itertools;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::views::LemmaWithKonsepView;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, diff::Diff, ts_rs::TS)]
#[ts(export, export_to = "../../src/bindings/")]
#[diff(attr(
    #[derive(Debug)]
))]
pub struct LemmaItem {
    pub id: i32,
    pub lemma: String,
    pub konseps: Vec<KonsepItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, diff::Diff, ts_rs::TS)]
#[ts(export, export_to = "../../src/bindings/")]
#[diff(attr(
    #[derive(Debug)]
))]
pub struct KonsepItem {
    pub id: i32,
    pub keterangan: String,
    pub golongan_kata: String,
    pub cakupans: Vec<String>,
    pub kata_asing: Vec<KataAsingItem>,
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

impl LemmaItem {
    pub fn from_views(views: Vec<LemmaWithKonsepView>) -> Vec<LemmaItem> {
        let mut data = Vec::<LemmaItem>::new();
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
            let mut kvec = Vec::<KonsepItem>::new();
            for (konsep, views) in konsep_map.iter() {
                kvec.push(KonsepItem {
                    id: konsep.0.clone(),
                    keterangan: konsep.1.clone(),
                    golongan_kata: views[0].golongan_kata.clone().unwrap_or("".into()).clone(),
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
                            a.push(KataAsingItem { nama, bahasa });
                        }
                        a
                    }),
                })
            }
            data.push(LemmaItem {
                id: lemma.0.clone(),
                lemma: lemma.1.clone(),
                konseps: kvec,
            })
        }
        data
    }
}
