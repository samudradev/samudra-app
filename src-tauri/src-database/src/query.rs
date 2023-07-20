use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    data::LemmaData,
    models::{self, prelude::*},
};

#[derive(Default)]
pub struct Query {
    lemma: String,
}

impl Query {
    pub fn new() -> Query {
        Default::default()
    }

    pub fn lemma(mut self, lemma: &str) -> Self {
        self.lemma = lemma.to_string();
        self
    }

    pub async fn collect(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Vec<LemmaData>, sea_orm::error::DbErr> {
        Ok(Lemma::find()
            .filter(models::lemma::Column::Nama.eq(self.lemma.clone()))
            .find_with_related(Konsep)
            .all(db)
            .await?
            .into_iter()
            .map(|v| LemmaData::from(v))
            .collect::<Vec<LemmaData>>())
    }
}
