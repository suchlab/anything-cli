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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use tempfile::TempDir;

    fn setup_test_environment() -> (TempDir, String) {
        let temp_dir = TempDir::new().unwrap();
        let fake_home = temp_dir.path().to_str().unwrap().to_string();
        (temp_dir, fake_home)
    }

    #[test]
    fn test_load_config_file_does_not_exist() {
        let (_temp_dir, fake_home) = setup_test_environment();

        // Temporarily set HOME environment variable
        let original_home = env::var("HOME").ok();
        env::set_var("HOME", &fake_home);

        let (config, path) = load_config("test-app");

        // Restore original HOME
        match original_home {
            Some(home) => env::set_var("HOME", home),
            None => env::remove_var("HOME"),
        }

        assert!(config.is_none());
        assert!(path.to_string_lossy().contains("test-app"));
        assert!(path.to_string_lossy().ends_with("config.json"));
    }

    #[test]
    fn test_load_config_valid_file() {
        let (_temp_dir, fake_home) = setup_test_environment();

        // Create config directory and file
        let config_dir = PathBuf::from(&fake_home).join(".test-app");
        fs::create_dir_all(&config_dir).unwrap();

        let config_path = config_dir.join("config.json");
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), "Bearer test".to_string());

        let test_config = Config {
            base_url: "https://api.test.com".to_string(),
            headers: Some(headers),
        };

        let config_content = serde_json::to_string_pretty(&test_config).unwrap();
        fs::write(&config_path, config_content).unwrap();

        // Temporarily set HOME environment variable
        let original_home = env::var("HOME").ok();
        env::set_var("HOME", &fake_home);

        let (config, path) = load_config("test-app");

        // Restore original HOME
        match original_home {
            Some(home) => env::set_var("HOME", home),
            None => env::remove_var("HOME"),
        }

        assert!(config.is_some());
        let loaded_config = config.unwrap();
        assert_eq!(loaded_config.base_url, "https://api.test.com");
        assert!(loaded_config.headers.is_some());

        let headers = loaded_config.headers.unwrap();
        assert_eq!(
            headers.get("Authorization"),
            Some(&"Bearer test".to_string())
        );
        assert_eq!(path, config_path);
    }

    #[test]
    fn test_load_config_invalid_json() {
        let (_temp_dir, fake_home) = setup_test_environment();

        // Create config directory and file with invalid JSON
        let config_dir = PathBuf::from(&fake_home).join(".test-app");
        fs::create_dir_all(&config_dir).unwrap();

        let config_path = config_dir.join("config.json");
        fs::write(&config_path, "{ invalid json }").unwrap();

        // Temporarily set HOME environment variable
        let original_home = env::var("HOME").ok();
        env::set_var("HOME", &fake_home);

        let (config, path) = load_config("test-app");

        // Restore original HOME
        match original_home {
            Some(home) => env::set_var("HOME", home),
            None => env::remove_var("HOME"),
        }

        assert!(config.is_none());
        assert_eq!(path, config_path);
    }

    #[test]
    fn test_load_config_empty_file() {
        let (_temp_dir, fake_home) = setup_test_environment();

        // Create config directory and empty file
        let config_dir = PathBuf::from(&fake_home).join(".test-app");
        fs::create_dir_all(&config_dir).unwrap();

        let config_path = config_dir.join("config.json");
        fs::write(&config_path, "").unwrap();

        // Temporarily set HOME environment variable
        let original_home = env::var("HOME").ok();
        env::set_var("HOME", &fake_home);

        let (config, path) = load_config("test-app");

        // Restore original HOME
        match original_home {
            Some(home) => env::set_var("HOME", home),
            None => env::remove_var("HOME"),
        }

        assert!(config.is_none());
        assert_eq!(path, config_path);
    }
}
