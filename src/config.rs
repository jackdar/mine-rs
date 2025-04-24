use serde::{Deserialize, Serialize};

// Define the server configuration structure using serde
#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub name: String,
    pub mc_version: String,
    pub jar: String,
    pub port: u16,
}
