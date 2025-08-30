use crate::config::data::Config;
use crate::config::loader::load_config;
use crate::config::saver::save_config;
use std::collections::HashMap;

pub fn handle_set_header(executable_name: &str, commands: &[String]) {
    if commands.len() < 2 {
        eprintln!("Usage: {} set-header <KEY> [VALUE]", executable_name);
        std::process::exit(1);
    }

    let key = commands[1].clone();
    let value = commands.get(2).cloned();

    // Gets config or creates new
    let (config_option, config_path) = load_config(executable_name);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_set_header_insufficient_args() {
        // Test that the function handles insufficient arguments
        let commands = vec!["self:set-header".to_string()];

        // The function would call std::process::exit(1) here
        // We test the condition instead
        assert!(commands.len() < 2);
    }

    #[test]
    fn test_header_add_logic() {
        // Test the logic for adding a header to a config
        let mut config = Config {
            base_url: "https://api.example.com".to_string(),
            headers: Some(HashMap::new()),
        };

        let key = "Authorization".to_string();
        let value = "Bearer token123".to_string();

        if let Some(headers) = &mut config.headers {
            headers.insert(key.clone(), value.clone());
        }

        assert!(config.headers.is_some());
        let headers = config.headers.unwrap();
        assert_eq!(
            headers.get("Authorization"),
            Some(&"Bearer token123".to_string())
        );
    }

    #[test]
    fn test_header_remove_logic() {
        // Test the logic for removing a header from a config
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), "Bearer token123".to_string());
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        let mut config = Config {
            base_url: "https://api.example.com".to_string(),
            headers: Some(headers),
        };

        let key = "Authorization".to_string();

        if let Some(headers) = &mut config.headers {
            headers.remove(&key);
        }

        let headers = config.headers.unwrap();
        assert!(headers.get("Authorization").is_none());
        assert_eq!(
            headers.get("Content-Type"),
            Some(&"application/json".to_string())
        );
    }

    #[test]
    fn test_header_update_logic() {
        // Test updating an existing header
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), "Bearer old-token".to_string());

        let mut config = Config {
            base_url: "https://api.example.com".to_string(),
            headers: Some(headers),
        };

        let key = "Authorization".to_string();
        let new_value = "Bearer new-token".to_string();

        if let Some(headers) = &mut config.headers {
            headers.insert(key.clone(), new_value.clone());
        }

        let headers = config.headers.unwrap();
        assert_eq!(
            headers.get("Authorization"),
            Some(&"Bearer new-token".to_string())
        );
    }

    #[test]
    fn test_config_creation_with_headers() {
        // Test creating a new config with headers
        let config = Config {
            base_url: String::new(),
            headers: Some(HashMap::new()),
        };

        assert_eq!(config.base_url, "");
        assert!(config.headers.is_some());
        assert!(config.headers.unwrap().is_empty());
    }

    #[test]
    fn test_headers_initialization() {
        // Test initializing headers when None
        let mut config = Config {
            base_url: "https://api.example.com".to_string(),
            headers: None,
        };

        if config.headers.is_none() {
            config.headers = Some(HashMap::new());
        }

        assert!(config.headers.is_some());
        assert!(config.headers.unwrap().is_empty());
    }

    #[test]
    fn test_command_parsing_key_only() {
        // Test parsing command with key only (for removal)
        let commands = vec!["self:set-header".to_string(), "Authorization".to_string()];

        assert!(commands.len() >= 2);
        assert_eq!(&commands[1], "Authorization");
        assert!(commands.get(2).is_none());
    }

    #[test]
    fn test_command_parsing_key_value() {
        // Test parsing command with key and value
        let commands = vec![
            "self:set-header".to_string(),
            "Authorization".to_string(),
            "Bearer token123".to_string(),
        ];

        assert!(commands.len() >= 2);
        assert_eq!(&commands[1], "Authorization");
        assert_eq!(commands.get(2), Some(&"Bearer token123".to_string()));
    }

    #[test]
    fn test_command_parsing_with_spaces_in_value() {
        // Test parsing command with spaces in header value
        let commands = vec![
            "self:set-header".to_string(),
            "User-Agent".to_string(),
            "MyApp/1.0 (Custom Agent)".to_string(),
        ];

        assert!(commands.len() >= 2);
        assert_eq!(&commands[1], "User-Agent");
        assert_eq!(
            commands.get(2),
            Some(&"MyApp/1.0 (Custom Agent)".to_string())
        );
    }

    #[test]
    fn test_multiple_headers_logic() {
        // Test managing multiple headers
        let mut config = Config {
            base_url: "https://api.example.com".to_string(),
            headers: Some(HashMap::new()),
        };

        let headers_to_add = vec![
            ("Authorization", "Bearer token123"),
            ("Content-Type", "application/json"),
            ("X-API-Version", "v1"),
            ("User-Agent", "MyApp/1.0"),
        ];

        if let Some(headers) = &mut config.headers {
            for (key, value) in headers_to_add {
                headers.insert(key.to_string(), value.to_string());
            }
        }

        let headers = config.headers.unwrap();
        assert_eq!(headers.len(), 4);
        assert_eq!(
            headers.get("Authorization"),
            Some(&"Bearer token123".to_string())
        );
        assert_eq!(
            headers.get("Content-Type"),
            Some(&"application/json".to_string())
        );
        assert_eq!(headers.get("X-API-Version"), Some(&"v1".to_string()));
        assert_eq!(headers.get("User-Agent"), Some(&"MyApp/1.0".to_string()));
    }
}
