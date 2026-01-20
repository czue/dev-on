use crate::config::Config;

pub fn execute(alias: &str) -> Result<(), String> {
    let mut config = Config::load()?;

    if config.remove_project(alias).is_none() {
        return Err(format!("Project '{}' not found", alias));
    }

    config.save()?;

    println!("Removed project '{}'", alias);
    Ok(())
}
