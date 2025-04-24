use std::{error::Error, fs, path::PathBuf};

use crate::{config::ServerConfig, docker::create_docker_compose};

#[derive(clap::Args)]
pub struct Args {
    /// Server name
    name: Option<String>,

    /// Server version
    #[arg(short, long, default_value = "1.21.5")]
    mc_version: String,

    /// Server jar
    #[arg(short, long, default_value = "paper")]
    jar: String,

    /// Server port
    #[arg(short, long, default_value = "25565")]
    port: u16,
}

fn get_server_dir(name: Option<String>) -> Result<PathBuf, Box<dyn Error>> {
    let dir = match name {
        Some(name) => {
            let path = PathBuf::from(name);

            if !path.exists() {
                fs::create_dir_all(&path)?;
                println!("Created new directory: {}", path.display());
            }

            path
        }
        None => std::env::current_dir()?,
    };

    Ok(dir)
}

fn print_server_info(server: &ServerConfig) -> Result<(), Box<dyn Error>> {
    // Print what we're doing
    println!("Creating Minecraft server: {}", server.name);
    println!("  Version: {}", server.mc_version);
    println!("  Jar: {}", server.jar);
    println!("  Port: {}", server.port);

    Ok(())
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let server_dir = get_server_dir(args.name)?;

    // Create server directory if it doesn't exist
    let is_empty = fs::read_dir(&server_dir)?.next().is_none();
    if !is_empty {
        println!(
            "Warning: Directory '{}' is not empty!",
            server_dir.display()
        );
        // TODO: Confirm creation with user.
    }

    let name = server_dir
        .file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap();

    let config = ServerConfig {
        name,
        mc_version: args.mc_version,
        jar: args.jar,
        port: 25565,
    };

    print_server_info(&config)?;

    // Convert config to TOML
    let toml_string = toml::to_string(&config).expect("Failed to serialize config");

    // Create directory if it doesn't exist
    fs::create_dir_all(&server_dir).expect("Failed to create server directory");

    // Write config to file
    fs::write(server_dir.join("server.toml"), toml_string).expect("Failed to write config file");

    // Create docker-compose file
    create_docker_compose(&server_dir, &config);

    println!("Server created at: {}", server_dir.display());
    println!(
        "To start the server: cd {} && docker-compose up -d",
        server_dir.display()
    );

    Ok(())
}
