use tauri::{Runtime, WindowMenuEvent};

use crate::menu::{DatabaseMenuItem, MenuItem};

pub fn on_menu_event(event: WindowMenuEvent) {
    _ = DatabaseMenuItem::on_menu_event(&event)
}
