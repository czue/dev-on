use crate::config::Config;
use crate::project::Project;
use std::path::PathBuf;

pub fn execute(alias: &str, path: &str, init_commands: Vec<String>) -> Result<(), String> {
    let mut config = Config::load()?;

    if config.get_project(alias).is_some() {
        return Err(format!("Project '{}' already exists", alias));
    }

    let path_buf = PathBuf::from(path);
    let project = Project::new(path_buf, init_commands);

    project.validate()?;

    config.add_project(alias.to_string(), project);
    config.save()?;

    println!("Added project '{}'", alias);
    Ok(())
}
