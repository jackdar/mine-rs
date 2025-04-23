use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

// Define the CLI arguments using clap
#[derive(Parser)]
#[command(author, version, about = "A simple Minecraft server manager", long_about = None)]
struct Cli {
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

// Define the server configuration structure using serde
#[derive(Serialize, Deserialize)]
struct ServerConfig {
    name: String,
    mc_version: String,
    jar: String,
    port: u16,
}

// Create a docker-compose file
fn create_docker_compose(dir: &Path, config: &ServerConfig) {
    // Define docker-compose as a string
    let docker_compose = format!(
        r#"services:
  minecraft:
    image: itzg/minecraft-server:latest
    container_name: mc-{}
    stdin_open: true
    tty: true
    ports:
      - {}:25565
    volumes:
      - ./data:/data
    environment:
      - EULA=TRUE
      - TYPE={}
      - VERSION={}
      - PAPER_CHANNEL=EXPERIMENTAL
    restart: unless-stopped
"#,
        config.name.to_lowercase(),
        config.port,
        config.jar.to_uppercase(),
        config.mc_version
    );

    // Write docker-compose file
    fs::write(dir.join("docker-compose.yml"), docker_compose)
        .expect("Failed to write docker-compose.yml file");
}

fn main() {
    // Parse CLI arguments
    let args = Cli::parse();

    let server_path = match args.name {
        Some(ref name) => {
            let path = PathBuf::from(name);

            if !path.exists() {
                fs::create_dir_all(&path).unwrap();
                println!("Created new directory: {}", path.display());
            }

            path
        }
        None => std::env::current_dir().unwrap(),
    };

    let is_empty = fs::read_dir(&server_path).unwrap().next().is_none();

    if !is_empty {
        println!(
            "Warning: Directory '{}' is not empty",
            server_path.display()
        );
        // TODO: Confirm creation with user.
    }

    let name = server_path
        .file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap();

    // Print what we're doing
    println!("Creating Minecraft server: {}", name);
    println!("  Version: {}", args.mc_version);
    println!("  Jar: {}", args.jar);
    println!("  Port: {}", args.port);

    // Create a server configuration
    let config = ServerConfig {
        name,
        mc_version: args.mc_version,
        jar: args.jar,
        port: 25565,
    };

    // Convert config to TOML
    let toml_string = toml::to_string(&config).expect("Failed to serialize config");

    // Create directory if it doesn't exist
    fs::create_dir_all(&server_path).expect("Failed to create server directory");

    // Write config to file
    fs::write(server_path.join("server.toml"), toml_string).expect("Failed to write config file");

    // Create docker-compose file
    create_docker_compose(&server_path, &config);

    println!("Server created at: {}", server_path.display());
    println!(
        "To start the server: cd {} && docker-compose up -d",
        server_path.display()
    );
}
