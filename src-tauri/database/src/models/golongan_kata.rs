use ormlite::types::chrono::{DateTime, Utc};
use ormlite::Model;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Model)]
#[ormlite(table = "golongan_kata", insertable=InsertGolonganKata)]
pub struct GolonganKata {
    #[ormlite(primary_key)]
    pub id: String,
    #[ormlite(default)]
    pub tarikh_masuk: DateTime<Utc>,
    pub nama: String,
    pub keterangan: String,
}

#[derive(Copy, Clone, Debug)]
pub enum Relation {
    // #[sea_orm(has_many = "super::konsep::Entity")]
    Konsep,
}
