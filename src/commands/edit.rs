use crate::config::Config;
use std::env;
use std::process::Command;

pub fn execute() -> Result<(), String> {
    let config_path = Config::config_path()?;

    if !config_path.exists() {
        return Err(format!(
            "Config file not found at {}. Run 'dev-on init' to create it.",
            config_path.display()
        ));
    }

    let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());

    let status = Command::new(&editor)
        .arg(&config_path)
        .status()
        .map_err(|e| format!("Failed to open editor '{}': {}", editor, e))?;

    if !status.success() {
        return Err(format!("Editor '{}' exited with error", editor));
    }

    Ok(())
}
