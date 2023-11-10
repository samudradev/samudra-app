use sqlx::{QueryBuilder, Sqlite};

use crate::views::LemmaWithKonsepView;

pub struct QueryView<'a, DB: sqlx::Database> {
    // view: V,
    query: QueryBuilder<'a, DB>,
}

impl<'a> QueryView<'a, Sqlite> {
    pub fn new() -> Self {
        let query = QueryBuilder::new(
            r#"SELECT 
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
        let mut query: QueryBuilder<sqlx::Sqlite> = self.query;
        let (empty_lemma, empty_konsep) = (params.lemma.is_empty(), params.konsep.is_empty());
        match (empty_lemma, empty_konsep) {
            (false, true) => {
                query.push(r#" WHERE lemma.nama = "#);
                query.push_bind(params.lemma);
            }
            (true, false) => {
                query.push(r#" WHERE konsep.keterangan LIKE "#);
                query.push_bind(params.konsep);
            }
            (false, false) => match params.mode {
                QueryMode::AND => {
                    query.push(r#" WHERE lemma.nama = "#);
                    query.push_bind(params.lemma);
                    query.push(r#" AND konsep.keterangan LIKE "#);
                    query.push_bind(params.konsep);
                }
                QueryMode::OR => {
                    query.push(r#" WHERE lemma.nama = "#);
                    query.push_bind(params.lemma);
                    query.push(r#" OR konsep.keterangan LIKE "#);
                    query.push_bind(params.konsep);
                }
            },
            (true, true) => return QueryView::new().all(db).await,
        };
        query.build_query_as().fetch_all(db).await
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
