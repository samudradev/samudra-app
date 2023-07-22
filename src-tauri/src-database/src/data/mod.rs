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
use serde::{Deserialize, Serialize}; // Required for the export! macro to work

use crate::models::cakupan::Model as Cakupan;
use crate::models::golongan_kata::Model as GolonganKata;
use crate::models::kata_asing::Model as KataAsing;
use crate::models::konsep::Model as Konsep;
use crate::models::lemma::Model as Lemma;

export! {
    LemmaData from Lemma {
    id: i32,
    nama: String;
    konseps: Konsep => ..KonsepData
    }
}

export! {
    KonsepData from Konsep {
    id: i32,
    // TODO This has different field from Konsep.golongan_id
    // golongan_kata: GolonganKataData,
    keterangan: Option<String>,
    tertib: Option<i32>;
    cakupan: Cakupan => ..CakupanData,
    kata_asing: KataAsing => ..KataAsingData
    }
}

export! {
    GolonganKataData from GolonganKata {
        id: String,
        nama: String,
        keterangan: String;
    }
}

export! {
    CakupanData from Cakupan {
    id: i32,
    nama: String,
    keterangan: Option<String>;
    }
}

export! {
    KataAsingData from KataAsing {
    id: i32,
    nama: String,
    bahasa: String;
    }
}
