//! A crate to handle database operations.

pub mod data;
pub mod errors;
pub mod insertions;
pub mod io;
pub mod operations;
pub mod states;
pub mod types;
pub mod views;

pub(crate) mod models;

pub mod prelude {
    // Datamodels
    pub(crate) use crate::data::items::kata_asing::KataAsingItem;
    pub(crate) use crate::data::items::konsep::KonsepItem;
    pub(crate) use crate::models::lemma::{InsertLemma, Lemma};
    pub(crate) use crate::views::LemmaWithKonsepView;

    // Types
    pub(crate) use crate::errors::BackendError;
    pub(crate) use crate::errors::Result;
    pub(crate) use crate::types::DbProvided;

    // Traits
    pub(crate) use crate::insertions::{ReferenceItem, ToTable, ToTableWithReference};
    pub(crate) use crate::models::JointTable;
    pub(crate) use itertools::Itertools;
    pub(crate) use ormlite::model::Insertable;
    pub(crate) use ormlite::Model;
}
