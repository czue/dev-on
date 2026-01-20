use crate::project::Project;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub projects: HashMap<String, Project>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            projects: HashMap::new(),
        }
    }

    pub fn config_path() -> Result<PathBuf, String> {
        dirs::home_dir()
            .map(|home| home.join(".devon.json"))
            .ok_or_else(|| "Could not determine home directory".to_string())
    }

    pub fn load() -> Result<Self, String> {
        let path = Self::config_path()?;

        if !path.exists() {
            return Err(format!(
                "Config file not found at {}. Run 'dev-on init' to create it.",
                path.display()
            ));
        }

        let contents = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        serde_json::from_str(&contents)
            .map_err(|e| format!("Failed to parse config file: {}", e))
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path()?;

        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        fs::write(&path, json)
            .map_err(|e| format!("Failed to write config file: {}", e))?;

        Ok(())
    }

    pub fn init_default() -> Result<(), String> {
        let path = Self::config_path()?;

        if path.exists() {
            return Err(format!(
                "Config file already exists at {}",
                path.display()
            ));
        }

        let config = Self::new();
        config.save()?;

        Ok(())
    }

    pub fn get_project(&self, name: &str) -> Option<&Project> {
        self.projects.get(name)
    }

    pub fn add_project(&mut self, name: String, project: Project) {
        self.projects.insert(name, project);
    }

    pub fn remove_project(&mut self, name: &str) -> Option<Project> {
        self.projects.remove(name)
    }

    pub fn list_projects(&self) -> Vec<&String> {
        let mut names: Vec<&String> = self.projects.keys().collect();
        names.sort();
        names
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
