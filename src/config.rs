use std::collections::HashMap;
use std::process;

use serde::{Deserialize, Serialize};

use crate::location::Location;
use crate::location::LocationProvider;
use crate::weather::providers::WeatherProvider;

pub const RETRY_TIMEOUT: u64 = 5;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub format: String,
    pub interval: i32,
    pub weather: WeatherConfig,
    pub location: LocationConfig,
    pub icons: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct WeatherConfig {
    pub provider: WeatherProvider,
    pub api_key: String,
}

#[derive(Serialize, Deserialize)]
pub struct LocationConfig {
    pub provider: LocationProvider,
    #[serde(flatten)]
    pub location: Location,
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
