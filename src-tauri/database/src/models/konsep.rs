use ormlite::types::chrono::{DateTime, Utc};
use ormlite::Model;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Model)]
#[ormlite(table = "konsep", insertable=InsertKonsep)]
pub struct Konsep {
    #[ormlite(primary_key)]
    pub id: i64,
    #[ormlite(default)]
    pub tarikh_masuk: DateTime<Utc>,
    pub lemma_id: i64,
    pub golongan_id: String,
    pub keterangan: Option<String>,
    pub tertib: Option<i64>,
}
