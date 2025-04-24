use clap::{command, Parser, Subcommand};
use std::error::Error;

mod commands;
mod config;
mod docker;

// Define the CLI arguments using clap
#[derive(Parser)]
#[command(author, version, about = "A simple Minecraft server manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Create a new Minecraft server
    Create(commands::create::Args),

    /// List all servers
    List,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse CLI arguments
    let args = Cli::parse();

    match args.command {
        Command::Create(args) => commands::create::run(args)?,
        Command::List => commands::list::run()?,
    }

    Ok(())
}
