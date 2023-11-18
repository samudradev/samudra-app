use ormlite::{model::Insertable, Model};
use serde::{Deserialize, Serialize};
use sqlx::Database;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Model)]
#[ormlite(table = "cakupan_x_konsep")]
pub struct CakupanXKonsep {
    #[ormlite(primary_key)]
    pub konsep_id: i32,
    #[ormlite(primary_key)]
    pub cakupan_id: i32,
}

impl CakupanXKonsep {
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
    //     belongs_to = "super::cakupan::Entity",
    //     from = "Column::CakupanId",
    //     to = "super::cakupan::Column::Id",
    //     on_update = "Cascade",
    //     on_delete = "Cascade"
    // )]
    Cakupan,
    // #[sea_orm(
    //     belongs_to = "super::konsep::Entity",
    //     from = "Column::KonsepId",
    //     to = "super::konsep::Column::Id",
    //     on_update = "Cascade",
    //     on_delete = "Cascade"
    // )]
    Konsep,
}
