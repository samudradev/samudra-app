use crate::errors::Result;

#[async_trait::async_trait]
pub trait ToTable<DB: sqlx::Database> {
    type OUTPUT: ormlite::Model<DB> + Sized + Send + Sync;

    /// Insert values while checking for duplicates
    async fn insert_safe(self, pool: &sqlx::Pool<DB>) -> Result<Self::OUTPUT>;

    async fn remove(&self, pool: &sqlx::Pool<DB>) -> Result<()> {
        todo!("LOW PRIORITY Default implementation of ToTable")
    }
}
#[async_trait::async_trait]
pub trait ToTableWithReference<DB: sqlx::Database> {
    type OUTPUT: ormlite::Model<DB> + Sized + Send + Sync;
    type REFERENCE: ReferenceItem;

    /// Insert values while checking for duplicates with referred value from `reference`.
    async fn insert_safe_with_reference(
        self,
        reference: &Self::REFERENCE,
        pool: &sqlx::Pool<DB>,
    ) -> Result<Self::OUTPUT>;
}
pub trait ReferenceItem {
    type FIELD;
    fn reference_field(&self) -> Self::FIELD;
}
