use ormlite::types::chrono::{DateTime, Utc};
use ormlite::Model;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Model)]
#[ormlite(table = "konsep", insertable=InsertKonsep)]
pub struct Konsep {
    #[ormlite(primary_key)]
    pub id: i32,
    #[ormlite(default)]
    pub tarikh_masuk: DateTime<Utc>,
    pub lemma_id: i32,
    pub golongan_id: String,
    pub keterangan: Option<String>,
    pub tertib: Option<i32>,
}

#[derive(Copy, Clone, Debug)]
pub enum Relation {
    // #[sea_orm(
    //     belongs_to = "super::golongan_kata::Entity",
    //     from = "Column::GolonganId",
    //     to = "super::golongan_kata::Column::Id",
    //     on_update = "Cascade",
    //     on_delete = "SetNull"
    // )]
    GolonganKata,
    // #[sea_orm(
    //     belongs_to = "super::lemma::Entity",
    //     from = "Column::LemmaId",
    //     to = "super::lemma::Column::Id",
    //     on_update = "NoAction",
    //     on_delete = "SetDefault"
    // )]
    Lemma,
}
