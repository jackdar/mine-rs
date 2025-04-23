use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

// Define the CLI arguments using clap
#[derive(Parser)]
#[command(author, version, about = "A simple Minecraft server manager", long_about = None)]
struct Cli {
    /// Server name
    #[arg(short, long, default_value = "minecraft-server")]
    name: String,

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
fn create_docker_compose(dir: &PathBuf, config: &ServerConfig) {
    // Define docker-compose as a string
    let docker_compose = format!(
        r#"services:
  minecraft:
    image: itzg/minecraft-server:latest
    container_name: mc-{}
    ports:
      - {}:25565
    environment:
      - EULA=TRUE
      - TYPE={}
      - VERSION={}
      - PAPER_CHANNEL=EXPERIMENTAL
    volumes:
      - ./data:/data
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

    // Print what we're doing
    println!("Creating Minecraft server: {}", args.name);
    println!("  Version: {}", args.mc_version);
    println!("  Software: {}", args.jar);
    println!("  Port: {}", args.port);

    // Create a server configuration
    let config = ServerConfig {
        name: args.name,
        mc_version: args.mc_version,
        jar: args.jar,
        port: 25565,
    };

    // Convert config to TOML
    let toml_string = toml::to_string(&config).expect("Failed to serialize config");

    // Create directory if it doesn't exist
    let server_dir = PathBuf::from(&config.name);
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
}
