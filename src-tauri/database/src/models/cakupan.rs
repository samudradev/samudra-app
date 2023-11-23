use ormlite::types::chrono::{DateTime, Utc};
use ormlite::Model;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Model)]
#[ormlite(table = "cakupan", insertable=InsertCakupan)]
pub struct Cakupan {
    #[ormlite(primary_key)]
    pub id: i32,
    #[ormlite(default)]
    pub tarikh_masuk: DateTime<Utc>,
    pub nama: String,
    pub keterangan: Option<String>,
}
