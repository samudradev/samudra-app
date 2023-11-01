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

// Required for the export! macro to work
pub use diff::{Diff, OptionDiff, VecDiff, VecDiffType};
// ---

use crate::models::cakupan::Model as Cakupan;
use crate::models::golongan_kata::Model as GolonganKata;
use crate::models::kata_asing::Model as KataAsing;
use crate::models::konsep::Model as Konsep;
use crate::models::lemma::Model as Lemma;

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
