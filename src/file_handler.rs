use std::{
    fs::{self, File},
    io::{Result, prelude::*},
    path::PathBuf,
};

extern crate dirs;

pub fn init_folders() -> Result<()> {
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

pub fn init_configs() -> Result<()> {
    let data_dir = get_data_dir();
    let config;
    if data_dir.join("config.toml").exists() {
        let mut file = File::open("config.toml")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        config = contents;
    } else {
    }
    Ok(())
}
