use crate::config::Config;

pub fn execute() -> Result<(), String> {
    let config = Config::load()?;

    let projects = config.list_projects();

    if projects.is_empty() {
        eprintln!("No projects configured. Use 'dev-on add' to add projects.");
        return Ok(());
    }

    for name in projects {
        println!("{}", name);
    }

    Ok(())
}
