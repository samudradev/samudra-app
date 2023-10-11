//! Backend for the Samudra App.

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(unused_variables, dead_code)]
// #![warn(missing_docs)]

mod appstate;
mod event;
mod menu;

use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use appstate::AppConfig;
use tauri::api::dialog::MessageDialogBuilder;
use tauri::api::dialog::MessageDialogKind;
use tauri::State;

use database;
use database::data::LemmaData;
use database::data::Diff;
use database::query::Query;
use database::DatabaseConnection;

// TODO Show config

#[tauri::command]
fn active_database_url(config: State<'_, AppConfig>) -> String {
    config.get_active_database().full_url()
}

// TODO Show database statistic
#[tauri::command(async)]
async fn count_lemma(config: State<'_, AppConfig>) -> Result<(), String> {
    dbg!(config.get_active_database().count_lemma().await.unwrap());
    Ok(())
}

#[tauri::command(async)]
async fn insert_single_value(config: State<'_, AppConfig>, lemma: String, konsep: String) -> Result<(), String> {
    match config.get_active_database().connect().await {
        DatabaseConnection::Disconnected => {
            let msg = format!("{} does not exist.", &config.get_active_database().path);
            Err(dbg!(msg))
        }
        conn => {
            match (database::io::RowValue{ lemma, konsep}).insert(&conn).await
            {
                Ok(msg) => MessageDialogBuilder::new("Success!".to_string(), "Success".to_string())
                .kind(MessageDialogKind::Info)
                .show(|_a| {}),
                Err(e) => MessageDialogBuilder::new("Failure!".to_string(), e.to_string())
                .kind(MessageDialogKind::Error)
                .show(|_a| {}),
            }
            Ok(())
        }
    }
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
async fn import_from_csv(config: State<'_, AppConfig>, path: String) -> Result<(), String> {
    match config.get_active_database().connect().await {
        DatabaseConnection::Disconnected => {
            let msg = format!("{} does not exist.", &config.get_active_database().path);
            Err(dbg!(msg))
        }
        conn => {
            match database::io::import_from_csv(
                &std::path::Path::new(&path),
                Some(b'|'),
                Some(b'\n'),
                dbg!(&conn),
            )
            .await
            {
                Ok(msg) => MessageDialogBuilder::new("Success!".to_string(), msg)
                    .kind(MessageDialogKind::Info)
                    .show(|_a| {}),
                Err(e) => MessageDialogBuilder::new("Failure!".to_string(), e.to_string())
                    .kind(MessageDialogKind::Error)
                    .show(|_a| {}),
            }
            Ok(())
        }
    }
}

#[tauri::command(async)]
async fn get(config: State<'_, AppConfig>, lemma: &str) -> Result<Vec<LemmaData>, String> {
    match config.get_active_database().connect().await {
        DatabaseConnection::Disconnected => {
            let msg = format!("{} does not exist.", &config.get_active_database().path);
            Err(dbg!(msg))
        }
        conn => Ok(dbg!(Query::new()
            .lemma(lemma)
            .collect(&conn)
            .await
            .unwrap())),
    }
}

#[tauri::command(async)]
async fn submit_changes(config: State<'_, AppConfig>, old: LemmaData, new: LemmaData) -> Result<(), String> {
    println!("{:#?}", old.diff(&new));
    Ok(())
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .menu(menu::MenuBar::default().as_menu())
        .on_menu_event(event::on_menu_event)
        .manage(appstate::AppPaths::default())
        .manage(appstate::AppConfig::default())
        .invoke_handler(tauri::generate_handler![
            get,
            active_database_url,
            count_lemma,
            import_from_csv,
            insert_single_value,
            submit_changes
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
