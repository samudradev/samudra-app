use crate::appstate::{AppConfig, AppPaths, DatabaseConfig};
use tauri::api::dialog::FileDialogBuilder;
use tauri::{Manager, WindowMenuEvent};

pub fn on_menu_event(event: WindowMenuEvent) -> () {
    match event.menu_item_id() {
        "register_database" => handle_database_registration(event),
        _ => println!("{} not handled", event.menu_item_id()),
    }
}

// TODO CUrrently just handle in storage. Not arbitrary paths.
fn handle_database_registration(event: WindowMenuEvent) -> () {
    let manager = event.window().app_handle();

    FileDialogBuilder::new()
        .set_title("Select folder to store database")
        .set_directory(manager.state::<AppPaths>().root.as_path())
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
                conf.register_database(name.clone(), &manager.state::<AppPaths>())
                    .unwrap();
                conf.set_active(name)
                    .unwrap()
                    .to_toml(&manager.state::<AppPaths>().root.as_path())
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
