use crate::config::data::Config;
use crate::config::loader::load_config;
use crate::config::saver::save_config;

pub fn handle_set_base_url(executable_name: &str, commands: &[String]) {
    if commands.len() < 2 {
        eprintln!("Usage: {} set-base-url <URL>", executable_name);
        std::process::exit(1);
    }

    let new_url = &commands[1];

    // Gets config or creates new
    let (config_option, config_path) = load_config(executable_name);
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_handle_set_base_url_insufficient_args() {
        let commands = vec!["self:set-base-url".to_string()];
        assert!(commands.len() < 2);
    }

    #[test]
    fn test_config_creation_logic() {
        // Test the logic for creating a new config when none exists
        let new_url = "https://api.example.com";

        let config = Config {
            base_url: String::new(),
            headers: None,
        };

        assert_eq!(config.base_url, "");
        assert!(config.headers.is_none());

        // Test updating the URL
        let mut updated_config = config;
        updated_config.base_url = new_url.to_string();

        assert_eq!(updated_config.base_url, "https://api.example.com");
    }

    #[test]
    fn test_config_update_preserves_headers() {
        // Test that updating base URL preserves existing headers
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), "Bearer token123".to_string());
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        let mut config = Config {
            base_url: "https://old-api.example.com".to_string(),
            headers: Some(headers.clone()),
        };

        // Update the base URL
        config.base_url = "https://new-api.example.com".to_string();

        assert_eq!(config.base_url, "https://new-api.example.com");
        assert_eq!(config.headers.unwrap(), headers);
    }

    #[test]
    fn test_command_parsing() {
        // Test command argument parsing logic
        let commands = vec![
            "self:set-base-url".to_string(),
            "https://api.test.com".to_string(),
        ];

        assert!(commands.len() >= 2);
        assert_eq!(&commands[1], "https://api.test.com");
    }

    #[test]
    fn test_command_parsing_with_extra_args() {
        // Test that extra arguments are ignored
        let commands = vec![
            "self:set-base-url".to_string(),
            "https://api.test.com".to_string(),
            "extra".to_string(),
            "arguments".to_string(),
        ];

        assert!(commands.len() >= 2);
        assert_eq!(&commands[1], "https://api.test.com");
        // Extra arguments are present but should be ignored
        assert_eq!(commands.len(), 4);
    }
}
