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
    use crate::data::{KataAsingRepr, KonsepDataRepr, LemmaDataRepr};
    use crate::query::QueryView;
    use crate::views::LemmaWithKonsepView;

    #[sqlx::test(fixtures("lemma", "lemma_2"))]
    fn test_lemma_w_konsep_view(pool: sqlx::Pool<sqlx::Sqlite>) {
        let views: Vec<LemmaWithKonsepView> = QueryView::new().all(&pool).await.unwrap();
        let mut data: Vec<LemmaDataRepr> = LemmaDataRepr::from_views(views);
        assert_eq!(
            data.sort_by(|a, b| a.lemma.cmp(&b.lemma)),
            vec![
                LemmaDataRepr {
                    id: 2,
                    lemma: "ufuk peristiwa".into(),
                    konseps: vec![KonsepDataRepr {
                        id: 1,
                        keterangan: "sempadan terluar lohong hitam".into(),
                        golongan_kata: "NAMA".into(),
                        cakupans: vec!["Astrofizik".into(), "Teori Relativiti".into(),],
                        kata_asing: vec![
                            KataAsingRepr {
                                nama: "event horizon".into(),
                                bahasa: "english".into(),
                            },
                            KataAsingRepr {
                                nama: "event horizon".into(),
                                bahasa: "english".into(),
                            },
                        ],
                    },],
                },
                LemmaDataRepr {
                    id: 1,
                    lemma: "cakera tokokan".into(),
                    konseps: vec![KonsepDataRepr {
                        id: 2,
                        keterangan: "gas-gas dan debu yang mengelilingi lohong hitam".into(),
                        golongan_kata: "NAMA".into(),
                        cakupans: vec!["Astrofizik".into(), "Teori Relativiti".into(),],
                        kata_asing: vec![
                            KataAsingRepr {
                                nama: "accretion disk".into(),
                                bahasa: "english".into(),
                            },
                            KataAsingRepr {
                                nama: "accretion disk".into(),
                                bahasa: "english".into(),
                            },
                        ],
                    },],
                },
            ]
            .sort_by(|a, b| a.lemma.cmp(&b.lemma))
        );
    }
}
