use crate::errors::BackendError;
use std::collections::HashMap;

pub(crate) trait View {}

pub(crate) trait FromView: Item {
    type VIEW: View;
    fn from_views(views: &Vec<Self::VIEW>) -> Vec<Self>;
}

pub(crate) trait FromViewMap: Item {
    type KEY;
    type VALUE;
    fn from_viewmap(value: &HashMap<Self::KEY, Self::VALUE>) -> Vec<Self>;
}
pub(crate) trait IntoViewMap<V: View>: IntoIterator<Item = V> {
    type KEY;
    type VALUE;
    fn into_viewmap(self) -> HashMap<Self::KEY, Self::VALUE>;
}

pub(crate) trait Item: Sized {
    type IntoMod: ItemMod;
    fn modify_into(&self, other: &Self) -> Result<Self::IntoMod, BackendError>;

    /// Instantiates a new struct with no attachment.

    fn partial_from_mod(other: &Self::IntoMod) -> Self;
}

#[async_trait::async_trait]
pub(crate) trait SubmitItem<DB: sqlx::Database>: Item {
    async fn submit_full(&self, pool: &sqlx::Pool<DB>) -> sqlx::Result<()>;

    async fn submit_partial(&self, pool: &sqlx::Pool<DB>) -> sqlx::Result<()>;
}

pub(crate) trait ItemMod {
    type FromItem: Item<IntoMod = Self>;
    fn from_item(value: &Self::FromItem) -> Self;
}

#[async_trait::async_trait]
pub(crate) trait SubmitMod<DB: sqlx::Database>: ItemMod {
    async fn submit_mod(&self, pool: &sqlx::Pool<DB>) -> sqlx::Result<()>;
}
#[async_trait::async_trait]
pub(crate) trait AttachmentItemMod<P: Item, DB: sqlx::Database>: ItemMod {
    async fn submit_attachment_to(&self, parent: &P, pool: &sqlx::Pool<DB>) -> sqlx::Result<()>;

    // TODO: Solve this
    // async fn submit_multi_attachment_to(many_self: &[Self], parent: &P, pool: &sqlx::Pool<DB>) -> sqlx::Result<()> {
    //     for i in many_self.into_iter() {
    //         i.submit_attachment_to(parent, pool).await
    //     }
    //     Ok(())
    // }

    async fn submit_detachment_from(&self, parent: &P, pool: &sqlx::Pool<DB>) -> sqlx::Result<()>;
    // TODO: Solve this
    // async fn submit_multi_detachment_to(many_self: &[Self], parent: &P, pool: &sqlx::Pool<DB>) -> sqlx::Result<()> {
    //     for i in many_self.into_iter() {
    //         i.submit_detachment_from(parent, pool).await
    //     }
    //     Ok(())
    // }
    async fn submit_modification_with(&self, parent: &P, pool: &sqlx::Pool<DB>)
        -> sqlx::Result<()>;
}
