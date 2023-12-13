use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::api::path;
use tauri::App;

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
/// active = "default"
///
/// [databases.default]
/// path = "root/storage/default/samudra.db"
/// engine = "sqlite"
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    active: Mutex<String>,
    databases: Mutex<HashMap<String, DatabaseConfig>>,
}

impl From<&AppPaths> for AppConfig {
    fn from(value: &AppPaths) -> Self {
        match dbg!(value.databases_toml().exists()) {
            true => {
                let contents = match fs::read_to_string(value.databases_toml()) {
                    Ok(c) => c,
                    Err(e) => {
                        todo!("LOW PRIORITY, {}", e)
                    }
                };
                match toml::from_str(&contents) {
                    Ok(c) => c,
                    Err(E) => {
                        std::fs::copy(
                            value.databases_toml(),
                            PathBuf::from_iter(vec![
                                value.root.clone(),
                                "databases.bak.toml".into(),
                            ]),
                        )
                        .expect("Error occured while copying backup configuration.");
                        println!("An error occured while reading `./databases.toml`. Please check `./databases.bak.toml`.\n{}", E);
                        Self::fallback(&value)
                    }
                }
            }
            false => Self::fallback(&value),
        }
    }
}

impl AppConfig {
    fn null() -> Self {
        Self {
            active: "".to_string().into(),
            databases: HashMap::new().into(),
        }
    }

    /// Sets the default value with the provided path.
    pub fn fallback(paths: &AppPaths) -> AppConfig {
        let config = Self::null();
        let _ = config.register_database("default".into(), paths);
        let _ = config.set_active("default".into());
        let _ = config.to_toml(&paths.databases_toml());
        config
    }

    pub async fn connection(&self) -> Connection {
        Connection::from(self.get_active_database_url()).await
    }

    pub fn get_active_database_url(&self) -> String {
        let name = &self.active.lock().unwrap().to_string();
        let hashmap = dbg!(self.databases.lock().unwrap());
        hashmap.get(&name.clone()).unwrap().path.clone()
    }

    pub fn to_toml(&self, file: &Path) -> Result<(), std::io::Error> {
        let mut file_buf = fs::OpenOptions::new().write(true).create(true).open(file)?;
        file_buf.write_all(toml::to_string_pretty(self).unwrap().as_bytes())
    }

    pub fn register_database(&self, name: String, paths: &AppPaths) -> Result<(), tauri::Error> {
        self.databases.lock().unwrap().insert(
            name.clone(),
            DatabaseConfig::in_storage(name.clone(), paths),
        );
        Ok(())
    }

    pub fn set_active(&self, name: String) -> Result<&Self, String> {
        match self.databases.lock().unwrap().get(&name) {
            Some(_database) => {
                *self.active.lock().unwrap() = name.into();
                Ok(self)
            }
            None => Err(format!("Error while accessing the active name `{}`.", name)),
        }
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
    pub fn initialize_root_dir(self) -> Self {
        if !&self.root.exists() {
            std::fs::create_dir(&self.root).unwrap();
        }
        if !&self.storage_dir().exists() {
            std::fs::create_dir(&self.storage_dir()).unwrap();
        }
        self
    }
    pub fn databases_toml(&self) -> PathBuf {
        PathBuf::from_iter([self.root.clone(), "databases.toml".into()])
    }

    pub fn storage_dir(&self) -> PathBuf {
        PathBuf::from_iter([self.root.clone(), "storage".into()])
    }
}
