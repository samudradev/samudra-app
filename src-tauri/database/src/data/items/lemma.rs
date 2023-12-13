use crate::prelude::*;

use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, diff::Diff, ts_rs::TS)]
#[ts(export, export_to = "../../src/bindings/")]
#[diff(attr(
    #[derive(Debug)]
))]
pub struct LemmaItem {
    pub id: DbProvided<i32>,
    pub lemma: String,
    pub konseps: Vec<KonsepItem>,
}
impl ReferenceItem for LemmaItem {
    type FIELD = i32;
    fn reference_field(&self) -> Self::FIELD {
        match self.id {
            DbProvided::Known(v) => v,
            DbProvided::Unknown => todo!(),
        }
    }
}

#[async_trait::async_trait]
impl ToTable<sqlx::Sqlite> for LemmaItem {
    type OUTPUT = Lemma;
    async fn insert_safe(self, pool: &sqlx::Pool<sqlx::Sqlite>) -> Result<Lemma> {
        let lemma = match self.id {
            DbProvided::Known(v) => {
                Lemma::select()
                    .where_bind("id = ?", v)
                    .fetch_one(pool)
                    .await?
            }
            DbProvided::Unknown => match Lemma::select()
                .where_bind("nama = ?", self.lemma.clone())
                .fetch_optional(pool)
                .await?
            {
                Some(l) => l,
                None => {
                    InsertLemma {
                        nama: self.lemma.clone(),
                    }
                    .insert(pool)
                    .await?
                }
            },
        };
        let lemma_item = LemmaItem {
            id: DbProvided::Known(lemma.id),
            ..self.clone()
        };
        for kon in self.clone().konseps.into_iter().clone() {
            kon.insert_safe_with_reference(&lemma_item, pool).await?;
        }
        Ok(lemma)
    }
}

type KonsepHashMap = HashMap<(i32, String, String), Vec<LemmaWithKonsepView>>;
type LemmaWithKonsepHashMap = HashMap<(i32, String), KonsepHashMap>;

pub trait Item: Sized {
    type MAP;
    type VIEW;
    fn from_hashmap(value: &Self::MAP) -> Vec<Self>;
    fn from_views(value: &Vec<Self::VIEW>) -> Vec<Self>;
}

impl Item for KonsepItem {
    type MAP = KonsepHashMap;
    type VIEW = LemmaWithKonsepView;
    fn from_hashmap(value: &KonsepHashMap) -> Vec<Self> {
        let mut data = Vec::new();
        for (konsep, views) in value.into_iter() {
            data.push(KonsepItem {
                id: DbProvided::Known(konsep.0.clone()),
                keterangan: konsep.1.clone(),
                golongan_kata: konsep.2.clone(),
                cakupans: views.clone().into_iter().fold(vec![], |mut a, b| {
                    if let Some(c) = b.cakupan.clone() {
                        a.push(c.into());
                    }
                    a
                }),
                kata_asing: views.clone().into_iter().fold(vec![], |mut a, b| {
                    if let (Some(nama), Some(bahasa)) =
                        (b.kata_asing.clone(), b.bahasa_asing.clone())
                    {
                        a.push(KataAsingItem { nama, bahasa });
                    }
                    a
                }),
            })
        }
        data
    }

    fn from_views(value: &Vec<LemmaWithKonsepView>) -> Vec<Self> {
        todo!("LOW PRIOIRITY")
    }
}

impl Item for LemmaItem {
    type MAP = LemmaWithKonsepHashMap;
    type VIEW = LemmaWithKonsepView;
    fn from_hashmap(value: &LemmaWithKonsepHashMap) -> Vec<LemmaItem> {
        let mut data = Vec::<LemmaItem>::new();
        for (lemma, konsep_map) in value.iter() {
            data.push(LemmaItem {
                id: DbProvided::Known(lemma.0.clone()),
                lemma: lemma.1.clone(),
                konseps: KonsepItem::from_hashmap(konsep_map),
            })
        }
        data
    }
    fn from_views(views: &Vec<LemmaWithKonsepView>) -> Vec<LemmaItem> {
        Self::from_hashmap(&views.into_hashmap())
    }
}

trait IntoHashMap: IntoIterator {
    type OUTPUT;
    fn into_hashmap(&self) -> Self::OUTPUT;
}

impl IntoHashMap for Vec<LemmaWithKonsepView> {
    type OUTPUT = LemmaWithKonsepHashMap;

    fn into_hashmap(&self) -> Self::OUTPUT {
        self.clone()
            .into_iter()
            .into_group_map_by(|a| (a.l_id, a.lemma.clone()))
            .into_iter()
            .map(|(k, v): ((i32, String), Vec<LemmaWithKonsepView>)| {
                (
                    k,
                    v.into_iter().into_group_map_by(|a| {
                        (
                            a.k_id,
                            a.konsep.as_ref().unwrap().clone(),
                            a.golongan_kata.clone().unwrap_or_default(),
                        )
                    }),
                )
            })
            .collect()
    }
}
