use std::{fs, path::Path};

use crate::config::ServerConfig;

// Create a docker-compose file
pub fn create_docker_compose(dir: &Path, config: &ServerConfig) {
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
