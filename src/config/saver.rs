use crate::config::data::Config;
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

    match fs::write(config_path, data) {
        Ok(_) => true,
        Err(e) => {
            eprintln!("Error writing config file: {}", e);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use tempfile::TempDir;

    #[test]
    fn test_save_config_success() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");

        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), "Bearer test".to_string());

        let config = Config {
            base_url: "https://api.example.com".to_string(),
            headers: Some(headers),
        };

        let result = save_config(&config, &config_path);
        assert!(result);
        assert!(config_path.exists());

        // Verify the content was written correctly
        let content = fs::read_to_string(&config_path).unwrap();
        let loaded_config: Config = serde_json::from_str(&content).unwrap();
        assert_eq!(loaded_config.base_url, config.base_url);
        assert_eq!(loaded_config.headers, config.headers);
    }

    #[test]
    fn test_save_config_creates_directory() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join("nested").join("config");
        let config_path = config_dir.join("config.json");

        let config = Config {
            base_url: "https://api.example.com".to_string(),
            headers: None,
        };

        // Directory doesn't exist yet
        assert!(!config_dir.exists());

        let result = save_config(&config, &config_path);
        assert!(result);
        assert!(config_dir.exists());
        assert!(config_path.exists());
    }

    #[test]
    fn test_save_config_no_headers() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");

        let config = Config {
            base_url: "http://localhost:3000".to_string(),
            headers: None,
        };

        let result = save_config(&config, &config_path);
        assert!(result);

        // Verify the content
        let content = fs::read_to_string(&config_path).unwrap();
        let loaded_config: Config = serde_json::from_str(&content).unwrap();
        assert_eq!(loaded_config.base_url, "http://localhost:3000");
        assert!(loaded_config.headers.is_none());
    }

    #[test]
    fn test_save_config_with_complex_headers() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");

        let mut headers = HashMap::new();
        headers.insert(
            "Authorization".to_string(),
            "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9".to_string(),
        );
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("X-API-Version".to_string(), "v1".to_string());
        headers.insert("X-Custom-Header".to_string(), "custom-value".to_string());

        let config = Config {
            base_url: "https://complex-api.example.com/v1".to_string(),
            headers: Some(headers.clone()),
        };

        let result = save_config(&config, &config_path);
        assert!(result);

        // Verify all headers are preserved
        let content = fs::read_to_string(&config_path).unwrap();
        let loaded_config: Config = serde_json::from_str(&content).unwrap();
        assert_eq!(loaded_config.headers.unwrap(), headers);
    }

    #[test]
    fn test_save_config_overwrites_existing() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");

        // Save initial config
        let config1 = Config {
            base_url: "https://api1.example.com".to_string(),
            headers: None,
        };
        assert!(save_config(&config1, &config_path));

        // Overwrite with new config
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), "Bearer new-token".to_string());

        let config2 = Config {
            base_url: "https://api2.example.com".to_string(),
            headers: Some(headers),
        };
        assert!(save_config(&config2, &config_path));

        // Verify the new config was saved
        let content = fs::read_to_string(&config_path).unwrap();
        let loaded_config: Config = serde_json::from_str(&content).unwrap();
        assert_eq!(loaded_config.base_url, "https://api2.example.com");
        assert!(loaded_config.headers.is_some());
    }
}
