//! Backend for the Samudra App.
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(unused_variables, dead_code)]
#![warn(missing_docs)]

mod appstate;
mod event;
mod menu;

use appstate::AppConfig;
use appstate::AppPaths;
use database::insertions::ToTable;
use database::states::Connection;
use database::states::Counts;
use database::views::LemmaWithKonsepView;
use tauri::State;

use database;
use database::data::{Item, LemmaItem};
use database::operations::DiffSumbittable;

// TODO FEAT Share picture
// TODO Manage errors gracefully

/// Exposes the active database URL to the frontend.
#[tauri::command(async)]
async fn active_database_url(config: State<'_, AppConfig>) -> Result<String, String> {
    Ok(config.get_active_database_url())
}

/// Display name for sharing
#[tauri::command(async)]
async fn get_display_name(config: State<'_, AppConfig>) -> Result<String, String> {
    Ok(config.get_display_name())
}

/// Set display name for sharing
#[tauri::command(async)]
async fn set_display_name(
    config: State<'_, AppConfig>,
    paths: State<'_, AppPaths>,
    name: String,
) -> Result<(), String> {
    config.set_display_name(name);
    config
        .to_toml(&paths.databases_toml())
        .expect("Writing to toml failed");
    Ok(())
}

#[tauri::command(async)]
async fn register_database_and_set_active(
    paths: State<'_, AppPaths>,
    config: State<'_, AppConfig>,
    name: String,
) -> Result<String, tauri::Error> {
    let config = config.register_database_and_set_active(name.clone(), &paths)?;
    let url = config.get_active_database_url();
    let _ = Connection::create_and_migrate(url.clone())
        .await
        .expect("Error migrating");
    Ok(url)
}

#[tauri::command(async)]
async fn count_items(config: State<'_, AppConfig>) -> Result<Counts, String> {
    Ok(config.connection().await.statistics().await.unwrap())
}

/// Returns all golongan_katas from the table
#[tauri::command(async)]
async fn get_golongan_kata_enumeration(
    config: State<'_, AppConfig>,
) -> Result<Vec<String>, String> {
    let res = config
        .connection()
        .await
        .get_golongan_kata_enumeration()
        .await
        .unwrap();
    Ok(res
        .iter()
        .map(|a| a.item.to_owned())
        .collect::<Vec<String>>())
}

/// Insert single value
#[tauri::command(async)]
async fn insert_lemma(config: State<'_, AppConfig>, item: LemmaItem) -> Result<(), String> {
    let conn = config.connection().await.pool;
    item.insert_safe(&conn).await.unwrap();
    Ok(())
}

/// Delete single lemma
#[tauri::command(async)]
async fn delete_lemma(config: State<'_, AppConfig>, item: LemmaItem) -> Result<(), String> {
    let conn = config.connection().await.pool;
    item.remove(&conn).await.unwrap();
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
    let conn = config.connection().await.pool;
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
) -> Result<(), String> {
    let db = config.connection().await.pool;
    old.submit_changes(&new, &db).await.unwrap();
    Ok(())
}

/// The entrypoint of this tauri app
#[tokio::main]
async fn main() {
    let paths = appstate::AppPaths::default().initialize_root_dir();
    let config = appstate::AppConfig::from(&paths);

    tauri::Builder::default()
        .menu(menu::MenuBar::default().as_menu())
        .on_menu_event(event::on_menu_event)
        .manage(paths)
        .manage(config)
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
