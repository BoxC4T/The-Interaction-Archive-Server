use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations/");

use serde::{Deserialize, Serialize};

use std::{fs, io::Write, path::PathBuf};
macro_rules! pub_struct {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        #[derive(Deserialize, Serialize, Debug)]
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}

extern crate toml;

pub fn init_folders() -> std::io::Result<()> {
    let data_dir = get_data_dir();
    fs::create_dir_all(&data_dir)?;
    let dirs = ["connection-pfps", "raw-interactions", "backups"];
    for dir in dirs {
        fs::create_dir_all(data_dir.join(dir))?;
    }
    Ok(())
}

pub fn get_data_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("tia-server")
}

pub fn init_files() -> std::io::Result<()> {
    let data_dir = get_data_dir();
    let config_path = get_config_file(&data_dir);

    if !config_path.exists() {
        let template = include_str!("../example.config.toml");

        let mut f = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&config_path)?;

        f.write_all(template.as_bytes())?;
    }

    Ok(())
}

pub fn get_config_file(data_dir: &PathBuf) -> PathBuf {
    data_dir.join("config.toml")
}

pub fn get_db_file(data_dir: &PathBuf) -> PathBuf {
    data_dir.join("tia.db")
}

pub fn init_db() -> core::result::Result<(), Box<dyn std::error::Error>> {
    let data_dir = get_data_dir();
    let db_path = get_db_file(&data_dir);
    let database_url = format!("sqlite://{}", db_path.display());

    let mut conn = SqliteConnection::establish(&database_url).unwrap();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    Ok(())
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let data_dir = get_data_dir();
    let config_path = get_config_file(&data_dir);
    if !config_path.exists() {
        init_folders()?;
        init_files()?;
    }

    let content = std::fs::read_to_string(&config_path)?;
    let mut config: Config = toml::from_str(&content)?;

    if config.system.setup == false {
        init_db()?;
        config.system.setup = true;
        let mut toml_string = toml::to_string(&config)?;
        toml_string += "#DONT TOUCH^ (Can fuck your shit up)"; //Is this the best to handle it? no, but do I want to rewrite everything now that i know comments get removed? also no
        fs::write(&config_path, toml_string)?;
    }

    Ok(config)
}

pub_struct!(Config {
    server_config: ServerConfig,
    api_config: APIConfig,
    system: System,
});

pub_struct!(ServerConfig {});

pub_struct!(APIConfig {
    enabled: bool,
    port: u64,
});
//Zed's formater is having a fit so this cant use the macro (for now)
#[derive(Deserialize, Serialize, Debug)]
pub struct System {
    pub setup: bool,
}
