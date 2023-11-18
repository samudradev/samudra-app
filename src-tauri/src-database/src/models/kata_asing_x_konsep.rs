use ormlite::Model;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Model)]
#[ormlite(table = "kata_asing_x_konsep")]
pub struct KataAsingXKonsep {
    #[ormlite(primary_key)]
    pub konsep_id: i32,
    #[ormlite(primary_key)]
    pub kata_asing_id: i32,
}

impl KataAsingXKonsep {
    pub fn insert_safe(self, pool: &sqlx::Pool<sqlx::Sqlite>) -> sqlx::Result<Self> {
        Ok(self
            .insert(pool)
            .on_conflict(ormlite::query_builder::OnConflict::Ignore)
            .model)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Relation {
    // #[sea_orm(
    //     belongs_to = "super::kata_asing::Entity",
    //     from = "Column::KataAsingId",
    //     to = "super::kata_asing::Column::Id",
    //     on_update = "Cascade",
    //     on_delete = "Cascade"
    // )]
    KataAsing,
    // #[sea_orm(
    //     belongs_to = "super::konsep::Entity",
    //     from = "Column::KonsepId",
    //     to = "super::konsep::Column::Id",
    //     on_update = "Cascade",
    //     on_delete = "Cascade"
    // )]
    Konsep,
}
