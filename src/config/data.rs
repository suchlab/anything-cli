use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub base_url: String,
    pub headers: Option<HashMap<String, String>>,
}
