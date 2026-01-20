use crate::config::Config;

pub fn execute() -> Result<(), String> {
    let config_path = Config::config_path()?;

    Config::init_default()?;

    println!("Created config file at {}", config_path.display());
    println!("Use 'dev-on add' to add projects or 'dev-on edit' to edit manually.");

    Ok(())
}
