use ormlite::types::chrono::{DateTime, Utc};
use ormlite::Model;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Model)]
#[ormlite(table = "kata_asing", insertable=InsertKataAsing)]
pub struct KataAsing {
    #[ormlite(primary_key)]
    pub id: i32,
    #[ormlite(default)]
    pub tarikh_masuk: DateTime<Utc>,
    pub nama: String,
    pub bahasa: String,
}
