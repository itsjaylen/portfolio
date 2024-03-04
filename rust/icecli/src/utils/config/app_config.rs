use std::{ env, fs, path::Path };

use config::{ Config, File, FileFormat };
use dotenvy::dotenv;
use serde::{ Deserialize, Serialize };

use crate::utils::{ file_utils::FileUtils, utils::is_debug_mode };

use super::{ preferences::Preferences, server::Server, weatherapi::WeatherAPI };

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub preferences: Preferences,
    pub server: Server,
    pub weatherapi: WeatherAPI,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            preferences: Preferences::default(),
            server: Server::default(),
            weatherapi: WeatherAPI::default(),
        }
    }
}

impl AppConfig {
    pub fn load_config() -> AppConfig {
        dotenv().ok();

        if is_debug_mode() {
        } else {
            if let Some(home_dir) = env::var_os("HOME") {
                let config_dir = Path::new(&home_dir).join(".icecli");
                let config_file = format!("{}/{}", config_dir.to_string_lossy(), "config.toml");

                if let Some(config_dir_str) = config_dir.to_str() {
                    let _ = FileUtils::directory_exists(config_dir_str, true);
                    let _ = FileUtils::file_exists(&config_file, true);

                    let builder = Config::builder()
                        .add_source(File::new(&config_file, FileFormat::Toml))
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

                            let weatherapi = WeatherAPI {
                                enabled: config.get_bool("weatherapi.enabled").unwrap_or(false),
                                api_key: config
                                    .get_string("weatherapi.api_key")
                                    .unwrap_or_else(|_| String::from("")),
                                debug: config.get_bool("weatherapi.debug").unwrap_or(false),
                            };

                            return AppConfig {
                                preferences,
                                server,
                                weatherapi,
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
    pub fn create_config(app_config: &AppConfig) {
        if is_debug_mode() {
        } else {
            if let Some(home_dir) = env::var_os("HOME") {
                let config_dir = Path::new(&home_dir).join(".icecli");
                let config_file = config_dir.join("config.toml");

                if let Some(config_dir_str) = config_dir.to_str() {
                    let _ = FileUtils::directory_exists(config_dir_str, true);
                    let _ = FileUtils::file_exists(config_file.to_str().unwrap(), true);

                    if let Err(e) = fs::write(&config_file, toml::to_string(&app_config).unwrap()) {
                        eprintln!("Error writing to config file: {}", e);
                    }
                } else {
                    eprintln!("Failed to convert config_dir to a string");
                }
            }
        }
    }
}
