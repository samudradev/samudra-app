use crate::models::JointTable;
use ormlite::Model;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Model)]
#[ormlite(table = "cakupan_x_konsep")]
pub struct CakupanXKonsep {
    #[ormlite(primary_key)]
    pub konsep_id: i32,
    #[ormlite(primary_key)]
    pub cakupan_id: i32,
}

impl<DB> JointTable<DB> for CakupanXKonsep
where
    DB: sqlx::Database,
    Self: Model<DB>,
{
}
