use crate::{
    changes::{AttachmentMod, CompareAttachable, FieldMod},
    data::{ KonsepItemMod},
    io::interface::{AttachmentItemMod, FromView, FromViewMap, IntoViewMap, Item, SubmitItem, ItemMod},
    prelude::*,
};

use std::collections::HashMap;
use crate::io::interface::SubmitMod;
use crate::states::{Pool, Sqlite};

use super::konsep::KonsepHashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, ts_rs::TS)]
#[ts(export, export_to = "../../src/bindings/")]
pub struct LemmaItem {
    pub id: AutoGen<i64>,
    pub lemma: String,
    pub konseps: Vec<KonsepItem>,
}

#[derive(Debug, Clone)]
pub struct LemmaItemMod {
    pub id: AutoGen<i64>,
    pub lemma: FieldMod<String>,
    pub konseps: AttachmentMod<KonsepItemMod>,
}

impl Item for LemmaItem {
    type IntoMod = LemmaItemMod;
    fn modify_into(&self, other: &Self) -> Result<Self::IntoMod> {
        if self.id != other.id {
            Err(BackendError {
                message: String::from("ID Assertion error"),
            })
        } else {
            Ok(LemmaItemMod {
                id: self.id.clone(),
                lemma: FieldMod::compare(self.lemma.clone(), other.lemma.clone()),
                konseps: self.compare_attachment(other.konseps.to_owned()),
            })
        }
    }

    fn partial_from_mod(other: &LemmaItemMod) -> Self {
        LemmaItem {
            id: other.id.clone(),
            lemma: other.lemma.value().to_string(),
            konseps: vec![],
        }
    }
}

impl ItemMod for LemmaItemMod {
    type FromItem = LemmaItem;

    fn from_item(value: &Self::FromItem) -> Self {
        Self {
            id: value.id.clone(),
            lemma: FieldMod::Fixed(value.lemma.clone()),
            konseps: AttachmentMod::from(value.konseps.clone()),
        }
    }
}

#[async_trait::async_trait]
impl SubmitMod<sqlx::Sqlite> for LemmaItemMod {
    async fn submit_mod(&self, pool: &Pool<Sqlite>) -> sqlx::Result<()> {
        let item = LemmaItem::partial_from_mod(self);
        item.submit_partial(pool).await?;
        for attached in self.konseps.attached.iter() {
            attached.submit_attachment_to(&item, pool).await?;
        };
        for detached in self.konseps.detached.iter() {
            detached.submit_detachment_from(&item, pool).await?;
        };
        for modified in self.konseps.modified.iter() {
            modified.submit_modification_with(&item, pool).await?;
        };
        Ok(())
    }
}

/// This PartialEq trait is mainly used for testing purposes.
/// Therefore, id comparison is ignored.
/// To compare changes, use Diff trait.
impl PartialEq for LemmaItem {
    fn eq(&self, other: &Self) -> bool {
        let konseps = Vec::from_iter(self.konseps.clone().into_iter());
        self.lemma == other.lemma
            && other
                .konseps
                .iter()
                .filter(|a| !konseps.contains(a))
                .collect_vec()
                .is_empty()
    }
}

#[async_trait::async_trait]
impl SubmitItem<sqlx::Sqlite> for LemmaItem {
    async fn submit_full(&self, pool: &sqlx::Pool<sqlx::Sqlite>) -> sqlx::Result<()> {
        let _=self.submit_partial(pool).await?;
        for konsep in self.konseps.iter() {
            KonsepItemMod::from_item(konsep).submit_attachment_to(self, pool).await?;
        }
        Ok(())
    }

    async fn submit_partial(&self, pool: &Pool<Sqlite>) -> sqlx::Result<()> {
        sqlx::query! {
            r#"INSERT or IGNORE INTO lemma (nama) VALUES (?)"#,
            self.lemma
        }
        .execute(pool)
        .await?;
        Ok(())
    }
}


impl FromViewMap for LemmaItem {
    type KEY = (i64, String);
    type VALUE = KonsepHashMap;

    fn from_viewmap(value: &HashMap<Self::KEY, Self::VALUE>) -> Vec<LemmaItem> {
        let mut data = Vec::<LemmaItem>::new();
        for (lemma, konsep_map) in value.iter() {
            data.push(LemmaItem {
                id: AutoGen::Known(lemma.0),
                lemma: lemma.1.clone(),
                konseps: KonsepItem::from_viewmap(konsep_map),
            })
        }
        data
    }
}
impl FromView for LemmaItem {
    type VIEW = LemmaWithKonsepView;

    fn from_views(views: &Vec<Self::VIEW>) -> Vec<LemmaItem> {
        Self::from_viewmap(&(views.clone().into_viewmap()))
    }
}
