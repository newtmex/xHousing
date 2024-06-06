use serde::Deserialize;

use crate::helpers::toml_into;

const CONFIG_FILE: &str = "config.toml";

#[derive(Deserialize)]
pub struct Config {
    pub gateway: String,

    pub contracts_owner_pem: String,
}

impl Config {
    // Deserializes config from file
    pub fn load_config() -> Self {
        toml_into(CONFIG_FILE)
    }

    // Returns the gateway
    pub fn gateway(&self) -> &str {
        &self.gateway[..]
    }
}
