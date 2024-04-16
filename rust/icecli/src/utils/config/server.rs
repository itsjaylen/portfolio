use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
    pub enabled: bool,
    pub port: u16,
    pub debug: bool,
    pub address: String
}

impl Default for Server {
    fn default() -> Self {
        Server {
            enabled: true,
            port: 1080,
            debug: true,
            address: "0.0.0.0".to_string(),
        }
    }
}
