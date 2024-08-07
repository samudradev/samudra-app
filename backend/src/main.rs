//! Backend for the Samudra App.
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(unused_variables, dead_code)]
#![warn(missing_docs)]

mod appstate;
mod event;
mod menu;

use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;
use std::sync::Mutex;

use appbuild::DesktopApp;
use appstate::AppConfig;
use appstate::AppConfigToml;
use appstate::AppPaths;
use appstate::IoError;
use database::data::LemmaItem;
use database::io::interface::{FromView, Item, SubmitItem, SubmitMod};
use database::states::Connection;
use database::states::Counts;
use database::views::LemmaWithKonsepView;
use menu::MenuBar;
use onc::phonotactics::tags::SyllableTags;
use tauri::plugin::TauriPlugin;
use tauri::Assets;
use tauri::Builder;
use tauri::Manager;
use tauri::Runtime;
use tauri::State;

// #[derive(serde::Deserialize, serde::Serialize)]
// #[serde(transparent)]
// struct PhonotacticToml(SyllableTags<String>);

// #[tauri::command]
// fn parse_phonotactic_toml(input: String) -> PhonotacticToml {
//     toml::from_str(&input).expect("Toml error")
// }

// TODO: Manage errors gracefully

/// Exposes the active database URL to the frontend.
#[tauri::command(async)]
async fn active_database_url(config: State<'_, AppConfig>) -> Result<String, String> {
    Ok(match config.get_active_database_url() {
        Ok(a) => a,
        Err(e) => {
            e.show();
            Default::default()
        }
    })
}

/// Display name for sharing
#[tauri::command(async)]
async fn get_display_name(config: State<'_, AppConfig>) -> Result<String, String> {
    Ok(match config.get_display_name() {
        Ok(a) => a,
        Err(e) => {
            e.show();
            Default::default()
        }
    })
}

/// Set display name for sharing
#[tauri::command(async)]
async fn set_display_name(config: State<'_, AppConfig>, name: String) -> Result<(), String> {
    config.set_display_name(name);
    Ok(())
}

#[tauri::command(async)]
async fn register_database_and_set_active(
    config: State<'_, AppConfig>,
    name: String,
) -> Result<String, tauri::Error> {
    config.register_database(name.clone(), &config.paths);
    config.set_active(name.clone());
    let url = config.get_active_database_url().unwrap();
    let _ = Connection::create_and_migrate(url.clone())
        .await
        .expect("Error migrating");
    Ok(url)
}

#[tauri::command(async)]
async fn count_items(config: State<'_, AppConfig>) -> Result<Counts, ()> {
    Ok(match config.connection().await {
        Ok(a) => a.statistics().await.unwrap(),
        Err(e) => {
            e.show();
            Counts::default()
        }
    })
    // Ok(config.connection().await.statistics().await.unwrap())
}

/// Returns all golongan_katas from the table
#[tauri::command(async)]
async fn get_golongan_kata_enumeration(
    config: State<'_, AppConfig>,
) -> Result<Vec<String>, String> {
    let mut res = config
        .connection()
        .await
        .unwrap()
        .get_golongan_kata_enumeration()
        .await
        .unwrap();
    if res.is_empty() {
        res = config
            .connection()
            .await
            .unwrap()
            .populate_with_presets()
            .await
            .expect("Error Populating preset")
            .get_golongan_kata_enumeration()
            .await
            .unwrap();
    }
    Ok(res
        .iter()
        .map(|a| a.item.to_owned())
        .collect::<Vec<String>>())
}

/// Insert single value
#[tauri::command(async)]
async fn insert_lemma(config: State<'_, AppConfig>, item: LemmaItem) -> Result<(), String> {
    let conn = config.connection().await.unwrap().pool;
    item.submit_full(&conn).await.unwrap();
    Ok(())
}

/// Delete single lemma
#[tauri::command(async)]
async fn delete_lemma(config: State<'_, AppConfig>, item: LemmaItem) -> Result<(), String> {
    let conn = config.connection().await.unwrap().pool;
    item.submit_partial_removal(&conn).await.unwrap();
    Ok(())
}

/// Import data from a csv file.
///
/// The csv is expected to be delimited with a pipe (`|`) to avoid conflict with the use of comma in a sentence.
/// Currently, the heading expected from the csv file is `lemma` and `konsep`.
///
/// Example:
/// ```csv
/// lemma   | konsep
/// data_1  | keterangan data 1
/// ```
#[tauri::command(async)]
async fn import_from_csv(_config: State<'_, AppConfig>, _path: String) -> Result<(), String> {
    todo!("LOW PRIORITY, Reimplement using views instead.")
    // match config.get_active_database().connect().await {
    //     DatabaseConnection::Disconnected => {
    //         let msg = format!("{} does not exist.", &config.get_active_database().path);
    //         Err(dbg!(msg))
    //     }
    //     conn => {
    //         match database::io::import_from_csv(
    //             &std::path::Path::new(&path),
    //             Some(b'|'),
    //             Some(b'\n'),
    //             dbg!(&conn),
    //         )
    //         .await
    //         {
    //             Ok(msg) => MessageDialogBuilder::new("Success!".to_string(), msg)
    //                 .kind(MessageDialogKind::Info)
    //                 .show(|_a| {}),
    //             Err(e) => MessageDialogBuilder::new("Failure!".to_string(), e.to_string())
    //                 .kind(MessageDialogKind::Error)
    //                 .show(|_a| {}),
    //         }
    //         Ok(())
    //     }
    // }
}

/// Get a vector of [`LemmaItem`] that matches the argument.
#[tauri::command(async)]
async fn get_lemma(config: State<'_, AppConfig>, lemma: &str) -> Result<Vec<LemmaItem>, String> {
    let conn = config.connection().await.unwrap().pool;
    let views = if lemma.is_empty() {
        LemmaWithKonsepView::query_all(&conn).await.unwrap()
    } else {
        LemmaWithKonsepView::query_lemma(lemma.into(), &conn)
            .await
            .unwrap()
    };
    Ok(LemmaItem::from_views(&views))
}

/// Submit lemma changes to database
#[tauri::command(async)]
async fn submit_changes(
    config: State<'_, AppConfig>,
    old: LemmaItem,
    new: LemmaItem,
) -> Result<(), ()> {
    let db = config.connection().await.unwrap().pool;
    old.modify_into(&new)
        .unwrap()
        .submit_mod(&db)
        .await
        .unwrap();
    Ok(())
}

mod appbuild {
    use tauri::{plugin::TauriPlugin, Manager, Runtime};

    use crate::appstate::{self, AppConfig};

    #[derive(Default)]
    struct LifeCycle {
        startup_fns: Vec<fn()>,
        shutdown_fns: Vec<fn()>,
        configure_fns: Vec<fn()>,
    }

    #[derive(Default)]
    pub struct DesktopApp {
        paths: appstate::AppPaths,
        lifecycle: LifeCycle,
    }

    impl DesktopApp {
        pub fn startup_with(mut self, on_startup: Vec<fn()>) -> Self {
            self.lifecycle.startup_fns = on_startup;
            self
        }
        pub fn shutdown_with(mut self, on_shutdown: Vec<fn()>) -> Self {
            self.lifecycle.shutdown_fns = on_shutdown;
            self
        }
        pub fn init<R: Runtime>(self) -> TauriPlugin<R> {
            tauri::plugin::Builder::new("samudra-desktop")
                .setup(move |app| {
                    if !self.paths.root.exists() {
                        self.paths.initialize_root_dir()?
                    }
                    self.lifecycle.startup_fns.iter().fold((), |_, f| f());
                    app.manage(AppConfig::new(self.paths));
                    app.manage(self.lifecycle);
                    Ok(())
                })
                .on_drop(|app| {
                    let cycle = app.state::<LifeCycle>();
                    cycle.inner().shutdown_fns.iter().fold((), |_, f| f());
                })
                .build()
        }
    }
}

pub trait Dialog {
    fn show(&self);
}

/// The entrypoint of this tauri app
#[tokio::main]
async fn main() {
    let appender = tracing_appender::rolling::never(".", "tracing.log");
    let (non_blocking_appender, _guard) = tracing_appender::non_blocking(appender);
    tracing_subscriber::fmt()
        .with_writer(non_blocking_appender)
        .init();

    tauri::Builder::default()
        .plugin(DesktopApp::default().init())
        .manage(AppConfigToml::default())
        .menu(MenuBar::default().into())
        .on_menu_event(event::on_menu_event)
        .invoke_handler(tauri::generate_handler![
            get_lemma,
            active_database_url,
            count_items,
            import_from_csv,
            insert_lemma,
            delete_lemma,
            submit_changes,
            register_database_and_set_active,
            get_golongan_kata_enumeration,
            get_display_name,
            set_display_name,
            // parse_phonotactic_toml
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
