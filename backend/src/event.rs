use tauri::WindowMenuEvent;

pub fn on_menu_event(event: WindowMenuEvent) {
    match event.menu_item_id() {
        // "register_database" => handle_database_registration(event),
        eventname => {
            event.window().emit(eventname, None::<String>).unwrap();
        }
    }
}
