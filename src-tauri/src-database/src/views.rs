#[derive(Debug, Clone, sqlx::FromRow, Default)]
pub struct LemmaWithKonsepView {
    pub lemma: String,
    pub konsep: Option<String>,
    pub golongan_kata: Option<String>,
    pub cakupan: Option<String>,
    pub kata_asing: Option<String>,
    pub bahasa_asing: Option<String>,
    // IDs
    pub l_id: i32,
    pub k_id: i32,
}

#[cfg(test)]
mod test {

    use crate::data::{KataAsingItem, KonsepItem, LemmaItem};
    use crate::query::{QueryParams, QueryView};
    use crate::views::LemmaWithKonsepView;
    use itertools::Itertools;

    #[sqlx::test(fixtures("lemma", "lemma_2"))]
    fn test_lemma_w_konsep_view(pool: sqlx::Pool<sqlx::Sqlite>) {
        let views: Vec<LemmaWithKonsepView> = QueryView::new().all(&pool).await.unwrap();
        let mut data = dbg!(LemmaItem::from_views(views)
            .into_iter()
            .sorted_by(|a, b| a.lemma.cmp(&b.lemma)));
        assert_eq!(
            data.next().unwrap(),
            LemmaItem {
                id: 1,
                lemma: "cakera tokokan".into(),
                konseps: vec![KonsepItem {
                    id: 1,
                    keterangan: "gas-gas dan debu yang mengelilingi lohong hitam".into(),
                    golongan_kata: "NAMA".into(),
                    cakupans: vec!["Astrofizik".into(), "Teori Relativiti".into(),],
                    kata_asing: vec![
                        KataAsingItem {
                            nama: "accretion disk".into(),
                            bahasa: "english".into(),
                        },
                        KataAsingItem {
                            nama: "accretion disk".into(),
                            bahasa: "english".into(),
                        },
                    ],
                },],
            },
        );
        assert_eq!(
            data.next().unwrap(),
            LemmaItem {
                id: 2,
                lemma: "ufuk peristiwa".into(),
                konseps: vec![KonsepItem {
                    id: 2,
                    keterangan: "sempadan terluar lohong hitam".into(),
                    golongan_kata: "NAMA".into(),
                    cakupans: vec!["Astrofizik".into(), "Teori Relativiti".into(),],
                    kata_asing: vec![
                        KataAsingItem {
                            nama: "event horizon".into(),
                            bahasa: "english".into(),
                        },
                        KataAsingItem {
                            nama: "event horizon".into(),
                            bahasa: "english".into(),
                        },
                    ],
                },],
            }
        );
    }

    #[sqlx::test(fixtures("lemma", "lemma_2"))]
    fn test_lemma_w_empty_konsep_view(pool: sqlx::Pool<sqlx::Sqlite>) {
        let param = QueryParams::either("cakera tokokan".into(), "".into());
        let views: Vec<LemmaWithKonsepView> = QueryView::new().with(param, &pool).await.unwrap();
        let mut data = dbg!(LemmaItem::from_views(views)
            .into_iter()
            .sorted_by(|a, b| a.lemma.cmp(&b.lemma)));
        assert_eq!(
            data.next(),
            Some(LemmaItem {
                id: 1,
                lemma: "cakera tokokan".into(),
                konseps: vec![KonsepItem {
                    id: 1,
                    keterangan: "gas-gas dan debu yang mengelilingi lohong hitam".into(),
                    golongan_kata: "NAMA".into(),
                    cakupans: vec!["Astrofizik".into(), "Teori Relativiti".into(),],
                    kata_asing: vec![
                        KataAsingItem {
                            nama: "accretion disk".into(),
                            bahasa: "english".into(),
                        },
                        KataAsingItem {
                            nama: "accretion disk".into(),
                            bahasa: "english".into(),
                        },
                    ],
                },],
            }),
        );
    }
}
