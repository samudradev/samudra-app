//! A crate to handle database operations.

pub mod data;
pub mod io;
pub mod operations;
pub mod query;
pub mod states;
pub mod views;

pub(crate) mod models;

pub use sea_orm as orm;

// TODO Reimplement check duplicate using diff
// #[async_trait]
// trait CheckDuplicateTrait<E>:
//     ActiveModelTrait<Entity = E> + ActiveModelBehavior + TryIntoModel<E::Model> + Sync + Send
// where
//     E: EntityTrait,
// {
//     async fn check<C>(self, db: &C) -> Result<Self, DbErr>
//     where
//         C: ConnectionTrait;

//     async fn insert_with_check<C>(
//         &self,
//         key: <E as EntityTrait>::Column,
//         db: &C,
//     ) -> Result<E::Model, DbErr>
//     where
//         <E as EntityTrait>::Model: IntoActiveModel<Self>,
//         C: ConnectionTrait,
//     {
//         match self.clone().check(db).await {
//             Err(_e) => todo!(),
//             Ok(am) => match am.get(key) {
//                 ActiveValue::Unchanged(_) => Ok(am.try_into_model()?),
//                 ActiveValue::NotSet => Ok(am.insert(db).await?),
//                 _ => todo!(),
//             },
//         }
//     }
// }
