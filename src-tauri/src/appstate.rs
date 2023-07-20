use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::api::path;

use database::DatabaseConfig;

#[derive(Debug)]
pub struct AppPaths {
    pub user_home: PathBuf,
    pub config_home: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    active: Mutex<String>,
    databases: Mutex<HashMap<String, DatabaseConfig>>,
}

impl AppConfig {
    pub fn get_active_database(&self) -> DatabaseConfig {
        let mut hashmap = HashMap::<String, DatabaseConfig>::new();
        for (k, v) in self.databases.lock().unwrap().iter() {
            hashmap.insert(k.clone(), v.clone());
        }
        hashmap
            .get(&self.active.lock().unwrap().to_string())
            .unwrap()
            .clone()
    }
    pub fn from_toml(file: &Path) -> AppConfig {
        match file.exists() {
            true => {
                let contents = match fs::read_to_string(file) {
                    Ok(c) => c,
                    Err(e) => {
                        todo!("{}", e)
                    }
                };
                toml::from_str(&contents).unwrap()
            }
            false => AppConfig::default(),
        }
    }

    // TODO implement
    pub fn to_toml(&self, file: &Path) -> Result<(), std::io::Error> {
        let mut file_buf = fs::OpenOptions::new().write(true).open(file)?;
        file_buf.write_all(toml::to_string_pretty(self).unwrap().as_bytes())?;
        Ok(())
    }

    pub fn register_database(&self, name: String, database: DatabaseConfig) {
        dbg!(self.databases.lock().unwrap()).insert(name, database);
    }

    pub fn set_active(&self, name: String) -> Result<&Self, String> {
        match self.databases.lock().unwrap().get(&name) {
            Some(database) => {
                *self.active.lock().unwrap() = name.into();
                Ok(self)
            }
            None => Err("Error".to_string()),
        }
    }
}

impl Default for AppPaths {
    fn default() -> Self {
        let user_home = path::home_dir().unwrap();
        let config_home = user_home.as_path().join(".samudra/databases.toml");
        AppPaths {
            user_home,
            config_home,
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig::from_toml(AppPaths::default().config_home.as_path())
    }
}
