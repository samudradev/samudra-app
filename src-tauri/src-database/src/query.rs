use sqlx::{QueryBuilder, Sqlite};

use crate::views::LemmaWithKonsepView;

pub struct QueryView<'a, DB: sqlx::Database> {
    // view: V,
    query: QueryBuilder<'a, DB>,
}

impl<'a> QueryView<'a, Sqlite> {
    pub fn new() -> Self {
        let query = QueryBuilder::new(
            r#" SELECT 
            lemma.nama AS lemma,
            konsep.keterangan AS konsep, 
            konsep.golongan_id AS golongan_kata,
            cakupan.keterangan AS cakupan,
            kata_asing.nama AS kata_asing,
            kata_asing.bahasa AS bahasa_asing,
            lemma.id AS l_id,
            konsep.id AS k_id
            FROM lemma
            LEFT JOIN konsep ON konsep.lemma_id = lemma.id 
            LEFT JOIN cakupan_x_konsep as cxk ON cxk.konsep_id = konsep.id 
            LEFT JOIN cakupan ON cakupan.id = cxk.cakupan_id
            LEFT JOIN kata_asing_x_konsep as kaxk ON kaxk.konsep_id = konsep.id 
            LEFT JOIN kata_asing ON kata_asing.id = kaxk.kata_asing_id"#,
        );
        QueryView {
            // view: LemmaWithKonsepView::default(),
            query,
        }
    }
    pub async fn all(self, db: &sqlx::SqlitePool) -> sqlx::Result<Vec<LemmaWithKonsepView>> {
        sqlx::query_as(self.query.sql()).fetch_all(db).await
    }
    pub async fn with(
        self,
        params: QueryParams,
        db: &sqlx::SqlitePool,
    ) -> sqlx::Result<Vec<LemmaWithKonsepView>> {
        let mut query = self.query;
        match params.mode {
            QueryMode::AND => {
                query.push_bind(r#"WHERE lemma.nama = ? AND konsep.keterangan LIKE ?"#)
            }
            QueryMode::OR => query.push_bind(r#"WHERE lemma.nama = ? OR konsep.keterangan LIKE ?"#),
        };
        sqlx::query_as(query.sql())
            .bind(params.lemma)
            .bind(params.konsep)
            .fetch_all(db)
            .await
    }
}

#[derive(Debug)]
pub struct QueryParams {
    lemma: String,
    konsep: String,
    mode: QueryMode,
}

#[derive(Debug)]
enum QueryMode {
    AND,
    OR,
}

impl QueryParams {
    pub fn either(lemma: String, konsep: String) -> QueryParams {
        QueryParams {
            mode: QueryMode::OR,
            lemma,
            konsep,
        }
    }
    pub fn both(lemma: String, konsep: String) -> QueryParams {
        QueryParams {
            mode: QueryMode::AND,
            lemma,
            konsep,
        }
    }
}
