use crate::config::data::Config;
use std::{env, fs, path::PathBuf};

pub fn load_config(executable_name: &str) -> (Option<Config>, PathBuf) {
    let home_dir = env::var("HOME").expect("Failed to get HOME directory");
    let config_path: PathBuf =
        PathBuf::from(format!("{}/.{}", home_dir, executable_name)).join("config.json");

    if !config_path.exists() {
        return (None, config_path);
    }

    match fs::read_to_string(&config_path) {
        Ok(content) => (serde_json::from_str(&content).ok(), config_path),
        Err(_) => (None, config_path),
    }
}
