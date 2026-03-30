use diesel::Connection;
use diesel::sqlite::SqliteConnection;
use std::{fs, io::Write, path::PathBuf};

extern crate diesel;
extern crate dirs;

pub fn init_folders() -> std::io::Result<()> {
    let data_dir = get_data_dir();
    fs::create_dir_all(&data_dir)?;
    fs::create_dir_all(get_raw_interactions_dir(&data_dir))?;
    fs::create_dir_all(get_pfp_dir(&data_dir))?;
    fs::create_dir_all(get_backup_dir(&data_dir))?;
    Ok(())
}

pub fn get_data_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("tia-server")
}

pub fn get_raw_interactions_dir(data_dir: &PathBuf) -> PathBuf {
    data_dir.join("raw-interactions")
}

pub fn get_pfp_dir(data_dir: &PathBuf) -> PathBuf {
    data_dir.join("connection-pfps")
}

pub fn get_backup_dir(data_dir: &PathBuf) -> PathBuf {
    data_dir.join("backups")
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

pub fn init_db() -> SqliteConnection {
    let data_dir = get_data_dir();
    let db_path = get_db_file(&data_dir);
    let database_url = format!("sqlite://{}", db_path.display());

    SqliteConnection::establish(&database_url).expect("Error creating/opening database")
}
