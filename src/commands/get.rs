use crate::config::Config;

pub fn execute(project_name: &str) -> Result<(), String> {
    let config = Config::load()?;

    let project = config
        .get_project(project_name)
        .ok_or_else(|| format!("Project '{}' not found", project_name))?;

    project.validate()?;

    println!("{}", project.format_output());
    Ok(())
}
