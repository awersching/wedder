use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process;

use directories::ProjectDirs;
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

impl Config {
    pub fn new() -> Self {
        let config_path = Config::get_config_path("wedder");
        let config = Config::load_config(&config_path);

        if config.weather.api_key == "" {
            println!("No API key");
            process::exit(1)
        }
        config
    }

    fn load_config(path: &PathBuf) -> Config {
        let config_string = match fs::read_to_string(path) {
            Ok(cfg_str) => cfg_str,
            Err(err) => match err.kind() {
                io::ErrorKind::NotFound => Config::no_config_file(),
                _ => Config::malformed_config()
            }
        };

        match toml::from_str(&config_string) {
            Ok(config) => config,
            Err(_) => Config::malformed_config()
        }
    }

    fn get_config_path(name: &str) -> PathBuf {
        let project = match ProjectDirs::from("rs", name, name) {
            Some(dir) => dir,
            None => Config::no_config_file()
        };

        [
            project.config_dir().to_str().unwrap(),
            &format!("{}.toml", name),
        ].iter().collect()
    }

    fn no_config_file() -> ! {
        println!("No config file");
        process::exit(1)
    }

    fn malformed_config() -> ! {
        println!("Malformed config file");
        process::exit(1)
    }
}
