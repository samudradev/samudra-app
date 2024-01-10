use crate::changes::{AttachmentMod, CompareAttachable, FieldMod};
use crate::io::interface::{AttachmentItemMod, FromView, FromViewMap, Item, ItemMod};
use crate::prelude::*;
use std::collections::HashMap;

use crate::data::items::cakupan::CakupanItem;
use crate::data::items::lemma::LemmaItem;
use crate::states::{Pool, Sqlite};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, ts_rs::TS)]
#[ts(export, export_to = "../../src/bindings/")]
pub struct KonsepItem {
    pub id: AutoGen<i64>,
    pub keterangan: String,
    pub golongan_kata: String,
    #[ts(type = "Array<string>")]
    pub cakupans: Vec<CakupanItem>,
    pub kata_asing: Vec<KataAsingItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KonsepItemMod {
    pub id: AutoGen<i64>,
    pub keterangan: FieldMod<String>,
    pub golongan_kata: FieldMod<String>,
    pub cakupans: AttachmentMod<CakupanItem>,
    pub kata_asing: AttachmentMod<KataAsingItem>,
}

impl ItemMod for KonsepItemMod {
    type FromItem = KonsepItem;

    fn from_item(value: &Self::FromItem) -> Self {
        Self {
            id: value.id.clone(),
            keterangan: FieldMod::Fixed(value.keterangan.clone()),
            golongan_kata: FieldMod::Fixed(value.golongan_kata.clone()),
            cakupans: AttachmentMod::from(value.cakupans.clone()),
            kata_asing: AttachmentMod::from(value.kata_asing.clone()),
        }
    }
}

impl KonsepItemMod {
    pub async fn submit_changes(&self, pool: &sqlx::Pool<sqlx::Sqlite>) -> sqlx::Result<()> {
        if let AutoGen::Known(t) = self.id {
            let keterangan = &self.keterangan.value();
            let golkat = &self.golongan_kata.value();
            let _result = sqlx::query! {
                "UPDATE konsep SET keterangan = ?, golongan_id = ? WHERE id == ?;", keterangan, golkat, t
            }.execute(pool).await?;
            let attachment_cakupan = &self.cakupans;
            let konsep_skeleton = KonsepItem::skeleton_from_mod(self);
            for attached in attachment_cakupan.attached.clone().into_iter() {
                attached
                    .submit_attachment_to(&konsep_skeleton, pool)
                    .await
                    .expect("Error!");
            }
            for detached in attachment_cakupan.detached.clone().into_iter() {
                detached
                    .submit_detachment_from(&konsep_skeleton, pool)
                    .await
                    .expect("Error!");
            }
            let attachment_kata_asing = &self.kata_asing;
            for attached in attachment_kata_asing.attached.clone().into_iter() {
                attached
                    .submit_attachment_to(&konsep_skeleton, pool)
                    .await
                    .expect("Error!");
            }
            for detached in attachment_kata_asing.detached.clone().into_iter() {
                detached
                    .submit_detachment_from(&konsep_skeleton, pool)
                    .await
                    .expect("Error!");
            }
            Ok(())
        } else {
            Ok(())
        }
    }
}
impl KonsepItem {
    pub fn null() -> Self {
        Self {
            id: AutoGen::Unknown,
            keterangan: "".into(),
            golongan_kata: "".into(),
            cakupans: vec![],
            kata_asing: vec![],
        }
    }

    /// Trivially construct data from KonsepItemMod without attachment data
    pub fn skeleton_from_mod(other: &KonsepItemMod) -> Self {
        Self {
            id: other.id.clone(),
            keterangan: other.keterangan.value().to_string(),
            golongan_kata: other.golongan_kata.value().to_string(),
            cakupans: vec![],
            kata_asing: vec![],
        }
    }
}

/// This PartialEq trait is mainly used for testing purposes.
/// Therefore, id comparison is ignored.
/// To compare changes, use Diff trait.
impl PartialEq for KonsepItem {
    fn eq(&self, other: &Self) -> bool {
        self.keterangan == other.keterangan
            && self.golongan_kata == other.golongan_kata
            // Necessary to ignore vector order
            && other
                .cakupans
                .iter()
                .filter(|&a| !self.cakupans.contains(a))
                .collect_vec()
                .is_empty()
            && other
                .kata_asing
                .iter()
                .filter(|&a| !self.kata_asing.contains(a))
                .collect_vec()
                .is_empty()
    }
}

#[async_trait::async_trait]
impl AttachmentItemMod<LemmaItem, sqlx::Sqlite> for KonsepItemMod {
    async fn submit_attachment_to(
        &self,
        parent: &LemmaItem,
        pool: &sqlx::Pool<sqlx::Sqlite>,
    ) -> sqlx::Result<()> {
        let konsep = KonsepItem::partial_from_mod(self);
        sqlx::query! {
            r#" INSERT or IGNORE INTO konsep (keterangan, lemma_id, golongan_id)
                VALUES (
                    ?,
                    (SELECT id FROM lemma WHERE lemma.nama = ?),
                    (SELECT id FROM golongan_kata WHERE golongan_kata.nama = ?)
                )
                "#,
                konsep.keterangan,
                parent.lemma,
                konsep.golongan_kata
        }
        .execute(pool)
        .await?;
        for cakupan in self.cakupans.attached.iter() {
            cakupan.submit_attachment_to(&konsep, pool).await?;
        }
        for cakupan in self.cakupans.detached.iter() {
            cakupan.submit_detachment_from(&konsep, pool).await?;
        }
        for cakupan in self.cakupans.modified.iter() {
            cakupan.submit_modification_with(&konsep, pool).await?;
        }
        for kata_asing in self.kata_asing.attached.iter() {
            kata_asing.submit_attachment_to(&konsep, pool).await?;
        }
        for kata_asing in self.kata_asing.detached.iter() {
            kata_asing.submit_detachment_from(&konsep, pool).await?;
        }
        for kata_asing in self.kata_asing.attached.iter() {
            kata_asing.submit_modification_with(&konsep, pool).await?;
        }
        Ok(())
    }
    async fn submit_detachment_from(
        &self,
        _parent: &LemmaItem,
        _pool: &sqlx::Pool<sqlx::Sqlite>,
    ) -> sqlx::Result<()> {
        todo!()
    }

    async fn submit_modification_with(
        &self,
        parent: &LemmaItem,
        pool: &Pool<Sqlite>,
    ) -> sqlx::Result<()> {
        let konsep = KonsepItem::partial_from_mod(self);
        sqlx::query! {
            r#" UPDATE konsep
                SET keterangan = ?, golongan_id = ?
                WHERE (
                    id = ?
                    AND
                    lemma_id = ?
                )
                "#,
                konsep.keterangan,
                konsep.golongan_kata,
                konsep.id,
                parent.id,
        }
        .execute(pool)
        .await?;
        for cakupan in self.cakupans.attached.iter() {
            cakupan.submit_attachment_to(&konsep, pool).await?;
        }
        for cakupan in self.cakupans.detached.iter() {
            cakupan.submit_detachment_from(&konsep, pool).await?;
        }
        for cakupan in self.cakupans.modified.iter() {
            cakupan.submit_modification_with(&konsep, pool).await?;
        }
        for kata_asing in self.kata_asing.attached.iter() {
            kata_asing.submit_attachment_to(&konsep, pool).await?;
        }
        for kata_asing in self.kata_asing.detached.iter() {
            kata_asing.submit_detachment_from(&konsep, pool).await?;
        }
        for kata_asing in self.kata_asing.attached.iter() {
            kata_asing.submit_modification_with(&konsep, pool).await?;
        }
        Ok(())
    }
}

type Key = (i64, Option<String>, Option<String>);
type Value = Vec<LemmaWithKonsepView>;
pub(crate) type KonsepHashMap = HashMap<Key, Value>;

impl Item for KonsepItem {
    type IntoMod = KonsepItemMod;
    fn modify_into(&self, other: &Self) -> Result<Self::IntoMod> {
        if self.id != other.id {
            return Err(BackendError {
                message: String::from("ID Assertion error"),
            });
        }
        Ok(KonsepItemMod {
            id: self.id.clone(),
            keterangan: FieldMod::compare(self.keterangan.clone(), other.keterangan.clone()),
            golongan_kata: FieldMod::compare(
                self.golongan_kata.clone(),
                other.golongan_kata.clone(),
            ),
            cakupans: self.compare_attachment(other.cakupans.clone()),
            kata_asing: self.compare_attachment(other.kata_asing.clone()),
        })
    }

    fn partial_from_mod(other: &Self::IntoMod) -> Self {
        KonsepItem {
            id: other.id.clone(),
            keterangan: other.keterangan.value().to_string(),
            golongan_kata: other.golongan_kata.value().to_string(),
            cakupans: vec![],
            kata_asing: vec![],
        }
    }
}

impl FromViewMap for KonsepItem {
    type KEY = Key;
    type VALUE = Value;
    fn from_viewmap(value: &KonsepHashMap) -> Vec<Self> {
        let mut data = Vec::new();
        for (konsep, views) in value.iter().filter(|((_, kon, _), _)| kon.is_some()) {
            data.push(KonsepItem {
                id: AutoGen::Known(konsep.0),
                keterangan: konsep
                    .1
                    .clone()
                    .expect("None should have been filtered out"),
                golongan_kata: konsep.2.clone().unwrap_or_default(),
                cakupans: CakupanItem::from_views(views),
                kata_asing: KataAsingItem::from_views(views),
            })
        }
        data
    }
}
