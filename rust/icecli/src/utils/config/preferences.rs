use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Preferences {
    pub theme: String,
    pub debug: bool,
    pub auto_save: bool,
    pub debug_logging: bool,
    pub error_logging: bool,
}

impl Default for Preferences {
    fn default() -> Self {
        Preferences {
            theme: String::from("Dark"),
            debug: true,
            auto_save: false,
            debug_logging: true,
            error_logging: true,
        }
    }
}
