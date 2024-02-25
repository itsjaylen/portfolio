use std::env;
use std::path::Path;
use dotenvy::dotenv;
use config::{ Config, File, FileFormat };
use serde::Deserialize;

use crate::utils::file_utils::FileUtils;

#[derive(Debug, Deserialize)]
pub struct Preferences {
    pub theme: String,
    pub debug: bool,
    pub auto_save: bool,
    pub debug_logging: bool,
    pub error_logging: bool,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub enabled: bool,
    pub port: u16,
    pub debug: bool,
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

impl Default for Server {
    fn default() -> Self {
        Server {
            enabled: true,
            port: 1080,
            debug: true,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub preferences: Preferences,
    pub server: Server,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            preferences: Preferences::default(),
            server: Server::default(),
        }
    }
}

impl AppConfig {
    pub fn load_config() -> AppConfig {
        dotenv().ok();
        let debug = env::var("DEBUG").unwrap_or_default().parse().unwrap_or(false);

        if debug {
            println!("{:?}", debug);
        } else {
            if let Some(home_dir) = env::var_os("HOME") {
                let config_dir = Path::new(&home_dir).join(".icecli");
                let config_file = format!("{}/{}", config_dir.to_string_lossy(), "config.ini");

                if let Some(config_dir_str) = config_dir.to_str() {
                    let _ = FileUtils::directory_exists(config_dir_str, true);
                    let _ = FileUtils::file_exists(&config_file, true);

                    let builder = Config::builder()
                        .add_source(File::new(&config_file, FileFormat::Ini))
                        .set_override("override", "1");

                    match builder.expect("Failed to build config").build() {
                        Ok(config) => {
                            let preferences = Preferences {
                                theme: config
                                    .get_string("preferences.theme")
                                    .unwrap_or_else(|_| String::from("DefaultTheme")),
                                debug: config.get_bool("debug").unwrap_or(false),
                                auto_save: config
                                    .get_bool("preferences.auto_save")
                                    .unwrap_or(false),
                                debug_logging: config
                                    .get_bool("preferences.debug_logging")
                                    .unwrap_or(false),
                                error_logging: config
                                    .get_bool("preferences.error_logging")
                                    .unwrap_or(false),
                            };

                            let server = Server {
                                enabled: config.get_bool("server.enabled").unwrap_or(false),
                                port: config.get_int("server.port").unwrap_or(0) as u16,
                                debug: config.get_bool("server.debug").unwrap_or(false),
                            };

                            return AppConfig {
                                preferences,
                                server,
                            };
                        }
                        Err(e) => {
                            eprintln!("Error building config: {}", e);
                        }
                    }
                } else {
                    eprintln!("Failed to convert config_dir to a string");
                }
            }
        }

        // Return the default AppConfig if there was an issue with loading the configuration
        AppConfig::default()
    }
}
