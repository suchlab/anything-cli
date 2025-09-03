use crate::config::config::Config;
use crate::config::loader::load_config;
use crate::config::saver::save_config;
use std::collections::HashMap;

pub fn handle_set_header(executable_name: &String, commands: &[String]) {
    if commands.len() < 2 {
        eprintln!("Usage: {} set-header <KEY> [VALUE]", executable_name);
        std::process::exit(1);
    }

    let key = commands[1].clone();
    let value = commands.get(2).cloned();

    // Gets config or creates new
    let (config_option, config_path) = load_config(&executable_name);
    let mut config = config_option.unwrap_or_else(|| Config {
        base_url: String::new(),
        headers: Some(HashMap::new()),
    });

    if config.headers.is_none() {
        config.headers = Some(HashMap::new());
    }

    match value {
        None => {
            if let Some(headers) = &mut config.headers {
                headers.remove(&key);
            }
        }
        Some(v) => {
            if let Some(headers) = &mut config.headers {
                headers.insert(key.clone(), v.clone());
            }
        }
    };

    if !save_config(&config, &config_path) {
        eprintln!("Error while saving the configuration.");
    }

    std::process::exit(0);
}
