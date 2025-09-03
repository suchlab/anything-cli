use crate::config::config::Config;
use crate::config::loader::load_config;
use crate::config::saver::save_config;

pub fn handle_set_base_url(executable_name: &String, commands: &[String]) {
    if commands.len() < 2 {
        eprintln!("Usage: {} set-base-url <URL>", executable_name);
        std::process::exit(1);
    }

    let new_url = &commands[1];

    // Gets config or creates new
    let (config_option, config_path) = load_config(&executable_name);
    let mut config = config_option.unwrap_or_else(|| Config {
        base_url: String::new(),
        headers: None,
    });

    config.base_url = new_url.clone();

    if !save_config(&config, &config_path) {
        eprintln!("Error while saving the configuration.");
    }

    std::process::exit(0);
}
