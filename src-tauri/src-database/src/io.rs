use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection};
use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Debug, Serialize, Deserialize)]
struct RowValue {
    lemma: String,
    konsep: String,
}

fn read_csv(
    path: &std::path::Path,
    delimiter: Option<u8>,
    terminator: Option<u8>,
) -> Result<Vec<RowValue>, csv::Error> {
    let delim = delimiter.unwrap_or(b',');
    let term = match terminator {
        Some(c) => csv::Terminator::Any(c),
        None => csv::Terminator::CRLF,
    };
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(delim)
        .terminator(term)
        .trim(csv::Trim::All)
        .from_path(path)?;

    rdr.deserialize().into_iter().collect()
}

pub async fn import_from_csv(
    path: &std::path::Path,
    delimiter: Option<u8>,
    terminator: Option<u8>,
    db: &DatabaseConnection,
) -> Result<(), sea_orm::DbErr> {
    let data = dbg!(read_csv(path, delimiter, terminator).unwrap());
    for d in data.iter() {
        let lemma = models::lemma::ActiveModel {
            nama: ActiveValue::Set(d.lemma.to_owned()),
            ..Default::default()
        };
        let lemma = lemma.insert(db).await?;
        let konsep = models::konsep::ActiveModel {
            lemma_id: ActiveValue::Set(lemma.id),
            golongan_id: ActiveValue::Set(None),
            keterangan: ActiveValue::Set(Some(d.konsep.to_owned())),
            ..Default::default()
        };
        konsep.insert(db).await?;
    }
    Ok(())
}
