//! Backend for the Samudra App.

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(unused_variables, dead_code)]
// #![warn(missing_docs)]

mod appstate;
mod event;
mod menu;

use appstate::AppConfig;
use tauri::State;

use database::data::LemmaData;
use database::query::Query;
use database::DatabaseConnection;

// TODO Expose database path to frontend
// TODO Show database statistic
// TODO Import from CSV
// TODO Show config

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

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .menu(menu::MenuBar::default().as_menu())
        .on_menu_event(event::on_menu_event)
        .manage(appstate::AppPaths::default())
        .manage(appstate::AppConfig::default())
        .invoke_handler(tauri::generate_handler![
            get //     command::search,
                //     command::get_lemma
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
