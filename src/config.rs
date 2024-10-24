use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub address: String,
    pub port: u16,
    pub database_path: String,
}

impl Config {
    pub fn load() -> Self {
        let content = std::fs::read_to_string("config.toml").unwrap();
        toml::from_str(&content).unwrap()
    }
}
