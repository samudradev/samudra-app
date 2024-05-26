use std::fmt::format;

use onc::phonotactics::{Faker, PhonotacticRule};
use rand::prelude::*;

use database::{data::LemmaItem, io::interface::FromView, views::LemmaWithKonsepView};

const CONFIG: &'_ str = r#"
definition.onset = [["sp", "spr", "sw", "sk", "skr", "st", "str", "kl", "fl", "bl", "pl", "pr", "kr", "gr", "tr", "dr", "kh", "sy", "gh", "ny", "ng", "v", "x", "q", "f", "y", "w", "m", "n", "p", "t", "c", "k", "b", "d", "j", "g", "s", "h", "l", "r"]]
definition.nucleus = [["a", "e", "i", "o", "u"]]
definition.coda = [["ks", "ns", "nk", "lf", "rt", "rd", "rt", "kh", "sy", "gh", "ng", "q", "f", "b", "m", "n", "p", "t", "k", "s", "h", "l", "r"]]
"#;

#[derive(serde::Deserialize)]
#[serde(transparent)]
struct PhonotacticToml(PhonotacticRule);

fn to_csv(views: &Vec<LemmaWithKonsepView>) -> String {
    let mut csv = String::from("lemma,konsep,golongan_kata,cakupan\n");
    let items = LemmaItem::from_views(views);
    for v in items {
        for k in v.konseps {
            csv.push_str(&format!(
                "{},{},{},{}\n",
                v.lemma,
                k.keterangan,
                k.golongan_kata,
                k.cakupans
                    .iter()
                    .map(|i| i.clone().to_string())
                    .collect::<Vec<String>>()
                    .join("::")
            ))
        }
    }
    csv
}

fn main() {
    let phonotactic: PhonotacticToml = toml::from_str(&CONFIG).expect("Toml error");

    let mut faker = Faker::default();
    let mut faker2 = Faker::default();

    let wordclasses = (0..3)
        .map(|_| phonotactic.0.generate_fake_word(2 as usize, &mut faker.rng))
        .collect::<Vec<String>>();
    let cakupan = (0..7)
        .map(|_| phonotactic.0.generate_fake_word(3 as usize, &mut faker.rng))
        .collect::<Vec<String>>();
    let words = (0..20)
        .map(|_| {
            phonotactic
                .0
                .generate_fake_word((1..5).choose(&mut faker.rng).unwrap_or(3), &mut faker.rng)
        })
        .collect::<Vec<String>>();
    let concepts = (0..20)
        .map(|_| {
            words
                .iter()
                .choose_multiple(&mut faker.rng, (2..6).choose(&mut faker2.rng).unwrap_or(5))
                .iter()
                .map(|&s| s.to_owned())
                .collect::<Vec<String>>()
                .join(" ")
        })
        .collect::<Vec<String>>();
    let views = words
        .iter()
        .enumerate()
        .map(|(n, w)| {
            Vec::from_iter(
                cakupan
                    .choose_multiple(&mut faker2.rng, (1..6).choose(&mut faker.rng).unwrap_or(1))
                    .map(|c| LemmaWithKonsepView {
                        l_id: n as i64,
                        lemma: w.clone(),
                        k_id: n as i64,
                        konsep: Some(concepts[n].clone()),
                        golongan_kata: wordclasses.choose(&mut faker.rng).cloned(),
                        cakupan: Some(c.clone()),
                        kata_asing: None,
                        bahasa_asing: None,
                    }),
            )
        })
        .flatten()
        .collect::<Vec<LemmaWithKonsepView>>();
    println!("{}", to_csv(&views))
}
