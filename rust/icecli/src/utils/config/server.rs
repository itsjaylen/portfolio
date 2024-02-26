use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Server {
    pub enabled: bool,
    pub port: u16,
    pub debug: bool,
}

impl Default for Server {
    fn default() -> Self {
        Server {
            enabled: true,
            port: 1080,
            debug: true,
        }
    }
}
