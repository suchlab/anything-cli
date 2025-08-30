use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Config {
    pub base_url: String,
    pub headers: Option<HashMap<String, String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_serialization() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), "Bearer token123".to_string());

        let config = Config {
            base_url: "https://api.example.com".to_string(),
            headers: Some(headers),
        };

        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: Config = serde_json::from_str(&serialized).unwrap();

        assert_eq!(config, deserialized);
    }

    #[test]
    fn test_config_serialization_no_headers() {
        let config = Config {
            base_url: "https://api.example.com".to_string(),
            headers: None,
        };

        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: Config = serde_json::from_str(&serialized).unwrap();

        assert_eq!(config, deserialized);
    }

    #[test]
    fn test_config_deserialization_from_json() {
        let json_str = r#"
        {
            "base_url": "https://api.test.com",
            "headers": {
                "X-API-Key": "secret123",
                "User-Agent": "test-agent"
            }
        }"#;

        let config: Config = serde_json::from_str(json_str).unwrap();

        assert_eq!(config.base_url, "https://api.test.com");
        assert!(config.headers.is_some());

        let headers = config.headers.unwrap();
        assert_eq!(headers.get("X-API-Key"), Some(&"secret123".to_string()));
        assert_eq!(headers.get("User-Agent"), Some(&"test-agent".to_string()));
    }

    #[test]
    fn test_config_deserialization_minimal() {
        let json_str = r#"{"base_url": "http://localhost:3000"}"#;
        let config: Config = serde_json::from_str(json_str).unwrap();

        assert_eq!(config.base_url, "http://localhost:3000");
        assert!(config.headers.is_none());
    }
}
