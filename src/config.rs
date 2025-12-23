use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use crate::error::FlowError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    pub entry: String,
    #[serde(default)]
    pub authors: Vec<String>,
    #[serde(default)]
    pub type_required: bool,
    #[serde(default)]
    pub packages: HashMap<String, String>, // alias -> "github.com/user/repo@ref"
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: "my-flow-project".to_string(),
            version: "0.1.0".to_string(),
            entry: "src/main.flow".to_string(),
            authors: vec![],
            type_required: false,
            packages: HashMap::new(),
        }
    }
}

impl ProjectConfig {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn save(&self, path: &Path) -> Result<(), FlowError> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| FlowError::rift(&format!("Failed to serialize config: {}", e), 0, 0))?;
            
        fs::write(path, json)
            .map_err(|e| FlowError::rift(&format!("Failed to write config file: {}", e), 0, 0))
    }
    
    pub fn load(path: &Path) -> Result<Self, FlowError> {
        let content = fs::read_to_string(path)
            .map_err(|e| FlowError::rift(&format!("Failed to read config file: {}", e), 0, 0))?;
            
        serde_json::from_str(&content)
            .map_err(|e| FlowError::glitch(&format!("Failed to parse config file: {}", e), 0, 0))
    }
}