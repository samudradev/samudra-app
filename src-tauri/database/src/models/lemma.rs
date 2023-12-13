use ormlite::types::chrono::{DateTime, Utc};
use ormlite::Model;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Model)]
#[ormlite(table = "lemma", insertable = InsertLemma)]
pub struct Lemma {
    #[ormlite(primary_key)]
    pub id: i64,
    #[ormlite(default)]
    pub tarikh_masuk: DateTime<Utc>,
    pub nama: String,
}
