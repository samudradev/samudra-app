use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::data::LemmaData;
use crate::models;
use crate::models::prelude::*;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct LemmaSQLUnit {
    pub(crate) lemma: String,
    pub(crate) konsep: Option<String>,
    pub(crate) golongan_kata: Option<String>,
    pub(crate) cakupan: Option<String>,
    pub(crate) kata_asing: Option<String>,
    pub(crate) bahasa_asing: Option<String>,
}

#[allow(dead_code)]
impl LemmaSQLUnit {
    pub async fn query_with(
        query: Query,
        pool: &sqlx::SqlitePool,
    ) -> sqlx::Result<Vec<LemmaSQLUnit>> {
        match query.mode {
            QueryMode::AND => {
                sqlx::query_as!(
                    LemmaSQLUnit,
                    r#" SELECT 
                            lemma.nama AS lemma,
                            konsep.keterangan AS konsep, 
                            konsep.golongan_id AS golongan_kata,
                            cakupan.keterangan AS cakupan,
                            kata_asing.nama AS kata_asing,
                            kata_asing.bahasa AS bahasa_asing
                        FROM lemma
                        LEFT JOIN konsep ON konsep.lemma_id = lemma.id 
                        LEFT JOIN cakupan_x_konsep as cxk ON cxk.konsep_id = konsep.id 
                        LEFT JOIN cakupan ON cakupan.id = cxk.cakupan_id
                        LEFT JOIN kata_asing_x_konsep as kaxk ON kaxk.konsep_id = konsep.id 
                        LEFT JOIN kata_asing ON kata_asing.id = kaxk.kata_asing_id
                        WHERE lemma.nama = ? AND konsep.keterangan LIKE ?
                        "#,
                    query.lemma,
                    query.konsep
                )
                .fetch_all(pool)
                .await
            }
            QueryMode::OR => {
                sqlx::query_as!(
                    LemmaSQLUnit,
                    r#"SELECT 
                    lemma.nama AS lemma,
                    konsep.keterangan AS konsep, 
                    konsep.golongan_id AS golongan_kata,
                    cakupan.keterangan AS cakupan,
                    kata_asing.nama AS kata_asing,
                kata_asing.bahasa AS bahasa_asing
                FROM lemma
                LEFT JOIN konsep ON konsep.lemma_id = lemma.id 
                LEFT JOIN cakupan_x_konsep as cxk ON cxk.konsep_id = konsep.id 
                LEFT JOIN cakupan ON cakupan.id = cxk.cakupan_id
                LEFT JOIN kata_asing_x_konsep as kaxk ON kaxk.konsep_id = konsep.id 
                LEFT JOIN kata_asing ON kata_asing.id = kaxk.kata_asing_id
                WHERE lemma.nama = ? OR konsep.keterangan LIKE ?
                "#,
                    query.lemma,
                    query.konsep
                )
                .fetch_all(pool)
                .await
            }
        }
    }
    pub async fn query_all(pool: &sqlx::SqlitePool) -> sqlx::Result<Vec<LemmaSQLUnit>> {
        sqlx::query_as!(
            LemmaSQLUnit,
            r#"SELECT 
                    lemma.nama AS lemma,
                    konsep.keterangan AS konsep, 
                    konsep.golongan_id AS golongan_kata,
                    cakupan.keterangan AS cakupan,
                    kata_asing.nama AS kata_asing,
                    kata_asing.bahasa AS bahasa_asing
                FROM lemma
                LEFT JOIN konsep ON konsep.lemma_id = lemma.id 
                LEFT JOIN cakupan_x_konsep as cxk ON cxk.konsep_id = konsep.id 
                LEFT JOIN cakupan ON cakupan.id = cxk.cakupan_id
                LEFT JOIN kata_asing_x_konsep as kaxk ON kaxk.konsep_id = konsep.id 
                LEFT JOIN kata_asing ON kata_asing.id = kaxk.kata_asing_id
                "#
        )
        .fetch_all(pool)
        .await
    }
}

#[derive(Default)]
pub struct Query {
    lemma: String,
    konsep: String,
    mode: QueryMode,
}

#[derive(Debug, Default)]
enum QueryMode {
    AND,
    #[default]
    OR,
}

impl Query {
    pub fn new() -> Query {
        Default::default()
    }

    pub fn either() -> Query {
        Query {
            mode: QueryMode::OR,
            ..Default::default()
        }
    }
    pub fn strict() -> Query {
        Query {
            mode: QueryMode::AND,
            ..Default::default()
        }
    }

    pub fn lemma(mut self, lemma: &str) -> Self {
        self.lemma = lemma.to_string();
        self
    }
    pub fn konsep(mut self, konsep: &str) -> Self {
        self.konsep = konsep.to_string();
        self
    }

    pub async fn collect(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Vec<LemmaData>, sea_orm::error::DbErr> {
        let _mode = &self.mode;
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
