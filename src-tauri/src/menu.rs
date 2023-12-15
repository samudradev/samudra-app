use tauri::{CustomMenuItem, Menu, Submenu};

pub struct MenuBar {
    menu: Menu,
}

impl MenuBar {
    pub fn as_menu(self) -> Menu {
        self.menu
    }
}

impl Default for MenuBar {
    fn default() -> Self {
        let mut database_submenu = Submenu::new(
            "Config",
            Menu::new()
                .add_item(CustomMenuItem::new("register_database", "Create database"))
                .add_item(CustomMenuItem::new("set_display_name", "Set display name")),
        );

        MenuBar {
            menu: Menu::new().add_submenu(database_submenu),
        }
    }
}
