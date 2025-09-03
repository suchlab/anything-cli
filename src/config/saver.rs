use crate::config::config::Config;
use std::{fs, path::PathBuf};

pub fn save_config(config: &Config, config_path: &PathBuf) -> bool {
    let data = match serde_json::to_string_pretty(config) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error serializing config: {}", e);
            return false;
        }
    };

    if let Some(parent) = config_path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            eprintln!("Error creating config directory {:?}: {}", parent, e);
            return false;
        }
    }

    match fs::write(&config_path, data) {
        Ok(_) => true,
        Err(e) => {
            eprintln!("Error writing config file: {}", e);
            false
        }
    }
}
