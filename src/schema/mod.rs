use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Instruction {
    pub action: String,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub error: Option<bool>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AnythingSchema {
    pub schema: String,
    pub instructions: Option<Vec<Instruction>>,
}

pub fn parse_anything_schema(json_str: &str) -> Option<AnythingSchema> {
    let parsed: AnythingSchema = match serde_json::from_str(json_str) {
        Ok(val) => val,
        Err(_) => {
            return None;
        }
    };

    if parsed.schema.starts_with("anything-cli/v0") && parsed.instructions.is_some() {
        Some(parsed)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_anything_schema_valid() {
        let json_str = r#"
        {
            "schema": "anything-cli/v0.1.0",
            "instructions": [
                {
                    "action": "ping"
                },
                {
                    "action": "print",
                    "content": "Hello, World!"
                }
            ]
        }"#;

        let result = parse_anything_schema(json_str);
        assert!(result.is_some());

        let schema = result.unwrap();
        assert_eq!(schema.schema, "anything-cli/v0.1.0");
        assert!(schema.instructions.is_some());

        let instructions = schema.instructions.unwrap();
        assert_eq!(instructions.len(), 2);
        assert_eq!(instructions[0].action, "ping");
        assert_eq!(instructions[1].action, "print");
        assert_eq!(instructions[1].content, Some("Hello, World!".to_string()));
    }

    #[test]
    fn test_parse_anything_schema_with_error_flag() {
        let json_str = r#"
        {
            "schema": "anything-cli/v0.2.0",
            "instructions": [
                {
                    "action": "execute",
                    "content": "echo 'test'",
                    "error": true
                },
                {
                    "action": "print",
                    "content": "Error message",
                    "error": false
                }
            ]
        }"#;

        let result = parse_anything_schema(json_str);
        assert!(result.is_some());

        let schema = result.unwrap();
        let instructions = schema.instructions.unwrap();
        assert_eq!(instructions[0].error, Some(true));
        assert_eq!(instructions[1].error, Some(false));
    }

    #[test]
    fn test_parse_anything_schema_minimal() {
        let json_str = r#"
        {
            "schema": "anything-cli/v0",
            "instructions": [
                {
                    "action": "none"
                }
            ]
        }"#;

        let result = parse_anything_schema(json_str);
        assert!(result.is_some());

        let schema = result.unwrap();
        let instructions = schema.instructions.unwrap();
        assert_eq!(instructions[0].action, "none");
        assert!(instructions[0].content.is_none());
        assert!(instructions[0].error.is_none());
    }

    #[test]
    fn test_parse_anything_schema_invalid_schema_version() {
        let json_str = r#"
        {
            "schema": "other-cli/v1.0.0",
            "instructions": [
                {
                    "action": "ping"
                }
            ]
        }"#;

        let result = parse_anything_schema(json_str);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_anything_schema_no_instructions() {
        let json_str = r#"
        {
            "schema": "anything-cli/v0.1.0"
        }"#;

        let result = parse_anything_schema(json_str);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_anything_schema_empty_instructions() {
        let json_str = r#"
        {
            "schema": "anything-cli/v0.1.0",
            "instructions": []
        }"#;

        let result = parse_anything_schema(json_str);
        assert!(result.is_some());

        let schema = result.unwrap();
        let instructions = schema.instructions.unwrap();
        assert!(instructions.is_empty());
    }

    #[test]
    fn test_parse_anything_schema_invalid_json() {
        let json_str = "{ invalid json }";
        let result = parse_anything_schema(json_str);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_anything_schema_missing_required_fields() {
        let json_str = r#"
        {
            "instructions": [
                {
                    "action": "ping"
                }
            ]
        }"#;

        let result = parse_anything_schema(json_str);
        assert!(result.is_none());
    }

    #[test]
    fn test_instruction_defaults() {
        let json_str = r#"
        {
            "schema": "anything-cli/v0.1.0",
            "instructions": [
                {
                    "action": "test"
                }
            ]
        }"#;

        let schema = parse_anything_schema(json_str).unwrap();
        let instruction = &schema.instructions.unwrap()[0];

        assert_eq!(instruction.action, "test");
        assert!(instruction.content.is_none());
        assert!(instruction.error.is_none());
    }

    #[test]
    fn test_parse_complex_schema() {
        let json_str = r#"
        {
            "schema": "anything-cli/v0.5.0",
            "instructions": [
                {
                    "action": "execute",
                    "content": "npm install",
                    "error": false
                },
                {
                    "action": "print",
                    "content": "Installation complete!"
                },
                {
                    "action": "execute",
                    "content": "npm test",
                    "error": true
                },
                {
                    "action": "none"
                }
            ]
        }"#;

        let result = parse_anything_schema(json_str);
        assert!(result.is_some());

        let schema = result.unwrap();
        assert_eq!(schema.schema, "anything-cli/v0.5.0");

        let instructions = schema.instructions.unwrap();
        assert_eq!(instructions.len(), 4);

        assert_eq!(instructions[0].action, "execute");
        assert_eq!(instructions[0].content, Some("npm install".to_string()));
        assert_eq!(instructions[0].error, Some(false));

        assert_eq!(instructions[1].action, "print");
        assert_eq!(
            instructions[1].content,
            Some("Installation complete!".to_string())
        );
        assert!(instructions[1].error.is_none());

        assert_eq!(instructions[2].action, "execute");
        assert_eq!(instructions[2].content, Some("npm test".to_string()));
        assert_eq!(instructions[2].error, Some(true));

        assert_eq!(instructions[3].action, "none");
        assert!(instructions[3].content.is_none());
        assert!(instructions[3].error.is_none());
    }
}
