use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct  WeatherAPI  {
    pub enabled: bool,
    pub api_key: String,
    pub debug: bool,
}

impl Default for WeatherAPI {
    fn default() -> Self {
      WeatherAPI {
            enabled: true,
            api_key: String::from(""),
            debug: true,
        }
    }
}
