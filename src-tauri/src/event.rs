use crate::appstate::{AppConfig, AppPaths};
use tauri::api::dialog::FileDialogBuilder;
use tauri::{Manager, WindowMenuEvent};

use database::DatabaseConfig;

pub fn on_menu_event(event: WindowMenuEvent) -> () {
    match event.menu_item_id() {
        "register_database" => handle_database_registration(event),
        _ => println!("{} not handled", event.menu_item_id()),
    }
}

fn handle_database_registration(event: WindowMenuEvent) -> () {
    let manager = event.window().app_handle();

    FileDialogBuilder::new()
        .set_title("Select folder to store database")
        .set_directory(manager.state::<AppPaths>().user_home.as_path())
        .add_filter("SQLite Database", &["db", "sqlite", "sqlite3"])
        .pick_folder(move |a| match a {
            Some(folder) => {
                let conf = manager.state::<AppConfig>();
                let name = folder
                    .components()
                    .last()
                    .unwrap()
                    .as_os_str()
                    .to_str()
                    .unwrap()
                    .to_string();
                conf.register_database(
                    name.clone(),
                    DatabaseConfig {
                        path: folder.join("samudra.db").to_str().unwrap().to_string(),
                        ..Default::default()
                    },
                )
                .unwrap();
                conf.set_active(name)
                    .unwrap()
                    .to_toml(&manager.state::<AppPaths>().config_home.as_path())
                    .unwrap();
                manager
                    .get_focused_window()
                    .unwrap()
                    .emit_and_trigger("active_database_changed", "")
                    .unwrap();
            }
            None => {}
        });
}
