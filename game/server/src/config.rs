use std::fs;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub client: ClientConfig,
    pub game: GameConfig,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub address: String,
}

#[derive(Deserialize)]
pub struct ClientConfig {

}

#[derive(Deserialize)]
pub struct GameConfig {
    update_interval: u32
}

impl Config {
    pub fn load(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_content = fs::read_to_string(file_path)?;
        let config: Config = toml::from_str(&config_content)?;
        Ok(config)
    }
}