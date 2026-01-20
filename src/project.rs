use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub path: PathBuf,
    #[serde(default)]
    pub init: Vec<String>,
}

impl Project {
    pub fn new(path: PathBuf, init: Vec<String>) -> Self {
        Self { path, init }
    }

    pub fn validate(&self) -> Result<(), String> {
        if !self.path.exists() {
            return Err(format!(
                "Project path does not exist: {}",
                self.path.display()
            ));
        }
        if !self.path.is_dir() {
            return Err(format!(
                "Project path is not a directory: {}",
                self.path.display()
            ));
        }
        Ok(())
    }

    pub fn format_output(&self) -> String {
        let mut output = self.path.display().to_string();
        for cmd in &self.init {
            output.push('|');
            output.push_str(cmd);
        }
        output
    }
}
