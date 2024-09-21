pub struct ListItemsView<T> {
    pub entries: Vec<T>,
}

#[cfg(feature = "sqlite")]
impl<T> crate::io::interface::View for ListItemsView<T> {
    type SOURCE = sqlx::Sqlite;
}

#[cfg(feature = "postgres")]
impl<T> crate::io::interface::View for ListItemsView<T> {
    type SOURCE = sqlx::Postgres;
}

#[cfg(any(feature = "sqlite", feature = "postgres"))]
impl ListItemsView<(String, String)> {
    pub async fn of_kata_asing(
        pool: &sqlx::Pool<<Self as crate::io::interface::View>::SOURCE>,
    ) -> sqlx::Result<Self> {
        Ok(Self {
            entries: sqlx::query!("SELECT bahasa, nama FROM kata_asing")
                .fetch_all(pool)
                .await?
                .iter()
                .map(|a| (a.bahasa.clone(), a.nama.clone()))
                .collect(),
        })
    }
}

#[cfg(any(feature = "sqlite", feature = "postgres"))]
impl ListItemsView<String> {
    pub async fn of_lemma(
        pool: &sqlx::Pool<<Self as crate::io::interface::View>::SOURCE>,
    ) -> sqlx::Result<Self> {
        Ok(Self {
            entries: sqlx::query!("SELECT nama FROM lemma")
                .fetch_all(pool)
                .await?
                .iter()
                .map(|a| a.nama.clone())
                .collect(),
        })
    }
    pub async fn of_golongan_kata(
        pool: &sqlx::Pool<<Self as crate::io::interface::View>::SOURCE>,
    ) -> sqlx::Result<Self> {
        Ok(Self {
            entries: sqlx::query!("SELECT nama FROM golongan_kata")
                .fetch_all(pool)
                .await?
                .iter()
                .map(|a| a.nama.clone())
                .collect(),
        })
    }
    pub async fn of_cakupan(
        pool: &sqlx::Pool<<Self as crate::io::interface::View>::SOURCE>,
    ) -> sqlx::Result<Self> {
        Ok(Self {
            entries: sqlx::query!("SELECT nama FROM cakupan")
                .fetch_all(pool)
                .await?
                .iter()
                .map(|a| a.nama.clone())
                .collect(),
        })
    }
}
