use std::{fs, path::{ PathBuf}};

extern crate dirs;

pub fn init_folders() -> Result<()> {
    let data_dir = get_data_dir();
    fs::create_dir_all(&data_dir)?;
    fs::create_dir_all(get_raw_interactions_dir())?;
    fs::create_dir_all(get_pfp_dir())?;
    fs::create_dir_all(get_backup_dir())?;
    Ok(())
}

pub fn get_data_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("tia-server")
}

pub fn get_raw_interactions_dir() -> PathBuf {
    get_data_dir().join("raw-interactions")
}

pub fn get_pfp_dir() -> PathBuf {
    get_data_dir().join("connection-pfps")
}

pub fn get_backup_dir -> PathBuf {
    get_data_dir().join("backups")
}
