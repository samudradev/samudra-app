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
        Self {
            path: apppaths
                .storage_dir()
                .join(name)
                .join("samudra.db")
                .to_str()
                .unwrap()
                .into(),
            engine: "sqlite".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    active: Mutex<String>,
    databases: Mutex<HashMap<String, DatabaseConfig>>,
}

impl From<&AppPaths> for AppConfig {
    fn from(value: &AppPaths) -> Self {
        match value.databases_toml().exists() {
            true => {
                let contents = match fs::read_to_string(value.databases_toml()) {
                    Ok(c) => c,
                    Err(e) => {
                        todo!("{}", e)
                    }
                };
                match toml::from_str(&contents) {
                    Ok(c) => c,
                    Err(E) => {
                        println!("An error occured while reading `./databases.toml`. Please check `./databases.bak.toml`.\n{}", E);
                        std::fs::copy(
                            value.databases_toml(),
                            PathBuf::from_iter(vec![
                                value.root.clone(),
                                "databases.bak.toml".into(),
                            ]),
                        )
                        .expect("Error occured while copying backup configuration.");
                        Self::fallback(&value)
                    }
                }
            }
            false => Self::fallback(&value),
        }
    }
}

impl AppConfig {
    pub fn fallback(value: &AppPaths) -> AppConfig {
        let mut databases = HashMap::new();
        databases.insert(
            "default".into(),
            DatabaseConfig::in_storage("default".into(), value),
        );
        Self {
            active: String::from("default").into(),
            databases: databases.into(),
        }
    }
    pub async fn connection(&self) -> Pool<Sqlite> {
        Connection::from(self.get_active_database_url()).await.pool
    }
    pub fn get_active_database_url(&self) -> String {
        let mut hashmap = HashMap::<String, DatabaseConfig>::new();
        for (k, v) in self.databases.lock().unwrap().iter() {
            hashmap.insert(k.clone(), v.clone());
        }
        let name = &self.active.lock().unwrap().to_string();
        hashmap.get(&name.clone()).unwrap().path.clone()
    }

    pub fn to_toml(&self, file: &Path) -> Result<(), std::io::Error> {
        let mut file_buf = fs::OpenOptions::new().write(true).open(file)?;
        file_buf.write_all(toml::to_string_pretty(self).unwrap().as_bytes())?;
        Ok(())
    }

    /// Currently only accepting default storage location
    pub fn register_database(&self, name: String, paths: &AppPaths) -> Result<(), tauri::Error> {
        let mut db_map = self.databases.lock().unwrap();

        db_map.insert(
            name.clone(),
            DatabaseConfig::in_storage(name.clone(), paths),
        );

        let db = db_map.get(&name).unwrap();

        tauri::async_runtime::block_on(async move {
            Connection::create_and_migrate(db.path.clone())
                .await
                .unwrap();
        });
        Ok(())
    }

    pub fn set_active(&self, name: String) -> Result<&Self, String> {
        match self.databases.lock().unwrap().get(&name) {
            Some(_database) => {
                *self.active.lock().unwrap() = name.into();
                Ok(self)
            }
            None => Err("Error".to_string()),
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
    pub fn databases_toml(&self) -> PathBuf {
        PathBuf::from_iter([self.root.clone(), "databases.toml".into()])
    }

    pub fn storage_dir(&self) -> PathBuf {
        PathBuf::from_iter([self.root.clone(), "storage".into()])
    }
}
