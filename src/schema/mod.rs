use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instruction {
    pub action: String,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub error: Option<bool>,
}

#[derive(Debug, Deserialize)]
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
