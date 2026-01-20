mod commands;
mod config;
mod project;

use clap::{Parser, Subcommand};
use std::process;

#[derive(Parser)]
#[command(name = "dev-on")]
#[command(about = "A project switcher for development environments", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Get project path and init commands")]
    Get {
        #[arg(help = "Project alias")]
        project: String,
    },
    #[command(about = "List all projects")]
    List,
    #[command(about = "Add a new project")]
    Add {
        #[arg(help = "Project alias")]
        alias: String,
        #[arg(help = "Project path")]
        path: String,
        #[arg(long, help = "Init command to run on activation")]
        init: Vec<String>,
    },
    #[command(about = "Remove a project")]
    Remove {
        #[arg(help = "Project alias")]
        alias: String,
    },
    #[command(about = "Edit config file")]
    Edit,
    #[command(about = "Initialize config file")]
    Init,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Get { project } => commands::get::execute(&project),
        Commands::List => commands::list::execute(),
        Commands::Add { alias, path, init } => commands::add::execute(&alias, &path, init),
        Commands::Remove { alias } => commands::remove::execute(&alias),
        Commands::Edit => commands::edit::execute(),
        Commands::Init => commands::init::execute(),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
