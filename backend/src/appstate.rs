use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::Infallible;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::{fs, io};
use tauri::api::path;
use tauri::utils::config;

pub use database::states::*;

#[derive(Debug)]
pub struct AppPaths {
    pub root: PathBuf,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseConfig {
    path: String,
    engine: String,
}

impl DatabaseConfig {
    pub fn in_storage(name: String, apppaths: &AppPaths) -> DatabaseConfig {
        let base = apppaths.storage_dir().join(name);
        if !base.exists() {
            std::fs::create_dir(&base).unwrap();
        }
        Self {
            path: base.join("samudra.db").to_str().unwrap().into(),
            engine: "sqlite".into(),
        }
    }
}

/// Represents the data in `databases.toml`
///
/// The default is initialized with [`AppConfig::fallback`]:
/// ```toml
/// display_name = "DISPLAY_NAME_UNSET"
/// active = "default"
///
/// [databases.default]
/// path = "root/storage/default/samudra.db"
/// engine = "sqlite"
/// ```
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AppConfigToml {
    display_name: String,
    active: String,
    databases: HashMap<String, DatabaseConfig>,
}

impl AppConfigToml {
    pub fn read(value: &PathBuf) -> Result<Self, IoError> {
        Ok(toml::from_str(&fs::read_to_string(value)?)?)
    }

    pub fn write(&self, file: &PathBuf) -> Result<(), std::io::Error> {
        let mut file_buf = fs::OpenOptions::new().write(true).create(true).open(file)?;
        file_buf.write_all(toml::to_string_pretty(self).unwrap().as_bytes())?;
        Ok(())
    }

    pub fn rewrite<F: FnMut(Self) -> Self>(path: &PathBuf, mut func: F) -> Result<(), IoError> {
        func(Self::read(path)?).write(path)?;
        Ok(())
    }
}

#[derive(Debug)]
enum IoError {
    Std(io::Error),
    Toml(toml::de::Error),
}

impl From<toml::de::Error> for IoError {
    fn from(value: toml::de::Error) -> Self {
        IoError::Toml(value)
    }
}

impl From<io::Error> for IoError {
    fn from(value: io::Error) -> Self {
        IoError::Std(value)
    }
}

#[derive(Debug, Default)]
pub struct AppConfig {
    pub paths: AppPaths,
}

impl AppConfig {
    pub fn new(paths: AppPaths) -> Self {
        Self { paths }
    }

    fn get_config(&self) -> AppConfigToml {
        AppConfigToml::read(&self.paths.databases_toml()).unwrap()
    }

    pub fn set_display_name(&self, name: String) {
        AppConfigToml::rewrite(&self.paths.databases_toml(), |mut conf| {
            conf.display_name = name.clone();
            conf
        });
    }
    pub fn get_display_name(&self) -> String {
        self.get_config().display_name
    }

    pub fn get_active_database_url(&self) -> String {
        let config = AppConfigToml::read(&self.paths.databases_toml()).unwrap();
        config.databases.get(&config.active).unwrap().path.clone()
    }

    pub async fn connection(&self) -> Connection {
        Connection::from(self.get_active_database_url()).await
    }

    pub fn register_database(&self, name: String, paths: &AppPaths) {
        AppConfigToml::rewrite(&self.paths.databases_toml(), |mut config| {
            config.databases.insert(
                name.clone(),
                DatabaseConfig::in_storage(name.clone(), paths),
            );
            config
        });
    }

    pub fn set_active(&self, name: String) {
        AppConfigToml::rewrite(&self.paths.databases_toml(), |mut config| {
            match config.databases.get(&name) {
                Some(_database) => {
                    config.active = name.clone();
                    config
                }
                None => panic!("Error while accessing the active name `{}`.", name),
            }
        })
        .unwrap();
    }
}

impl Default for AppPaths {
    fn default() -> Self {
        AppPaths {
            root: path::document_dir()
                .unwrap_or(path::home_dir().unwrap())
                .join("Samudra"),
        }
    }
}

impl AppPaths {
    pub fn initialize_root_dir(&self) -> Result<(), io::Error> {
        if !self.root.exists() {
            std::fs::create_dir(self.root.clone())?;
        }
        if !self.storage_dir().exists() {
            std::fs::create_dir(self.storage_dir())?;
        }
        if !self.export_dir().exists() {
            std::fs::create_dir(self.export_dir())?;
        }
        Ok(())
    }
    pub fn databases_toml(&self) -> PathBuf {
        PathBuf::from_iter([self.root.clone(), "databases.toml".into()])
    }

    pub fn storage_dir(&self) -> PathBuf {
        PathBuf::from_iter([self.root.clone(), "storage".into()])
    }
    pub fn export_dir(&self) -> PathBuf {
        PathBuf::from_iter([self.root.clone(), "export".into()])
    }
}
