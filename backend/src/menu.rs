use std::borrow::{Borrow, BorrowMut};

use tauri::{CustomMenuItem, Menu, Submenu, WindowMenuEvent};

pub struct MenuBar {
    menu: Menu,
}

impl Into<Menu> for MenuBar {
    fn into(self) -> Menu {
        self.as_menu()
    }
}

impl MenuBar {
    pub fn as_menu(self) -> Menu {
        self.menu
    }
}

pub enum DatabaseMenuItem {
    Register,
    SetUserName,
}

impl<'a> TryFrom<&'a str> for DatabaseMenuItem {
    type Error = &'a str;
    fn try_from(value: &'a str) -> Result<Self, &'a str> {
        match value {
            "database::register" => Ok(DatabaseMenuItem::Register),
            "database::set_username" => Ok(DatabaseMenuItem::SetUserName),
            a => Err(a),
        }
    }
}

impl MenuItem for DatabaseMenuItem {
    fn id(&self) -> &'static str {
        match self {
            DatabaseMenuItem::Register => "database::register",
            DatabaseMenuItem::SetUserName => "database::set_username",
        }
    }

    fn title(&self) -> &'static str {
        match self {
            DatabaseMenuItem::Register => "Create database",
            DatabaseMenuItem::SetUserName => "Set display name",
        }
    }

    fn on_menu_event<'a>(event: &'a WindowMenuEvent) -> Result<(), &'a str> {
        match Self::try_from(event.menu_item_id())? {
            DatabaseMenuItem::Register => todo!(),
            DatabaseMenuItem::SetUserName => todo!(),
        }
    }
}

pub trait MenuItem {
    fn id(&self) -> &'static str;
    fn title(&self) -> &'static str;
    fn as_custom_menu_item(&self) -> CustomMenuItem {
        CustomMenuItem::new(self.id(), self.title())
    }

    fn on_menu_event<'a>(event: &'a WindowMenuEvent) -> Result<(), &'a str>;
}

impl Default for MenuBar {
    fn default() -> Self {
        let database_submenu = Submenu::new(
            "Config",
            Menu::new()
                .add_item(DatabaseMenuItem::Register.as_custom_menu_item())
                .add_item(DatabaseMenuItem::SetUserName.as_custom_menu_item()),
        );

        MenuBar {
            menu: Menu::new().add_submenu(database_submenu),
        }
    }
}
