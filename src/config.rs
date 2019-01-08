use std::collections::HashMap;
use std::process;

use crate::weather::providers::WeatherProvider;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub weather_provider: WeatherProvider,
    pub weather_api_key: String,

    pub city: String,
    pub interval: i32,

    pub format: String,
    pub icons: HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        Config::no_config_file_found()
    }
}

impl Config {
    pub fn new() -> Self {
        match confy::load("wedder") {
            Ok(cfg) => cfg,
            Err(_) => Config::no_config_file_found()
        }
    }

    fn no_config_file_found() -> Self {
        println!("No config file");
        process::exit(1)
    }
}
