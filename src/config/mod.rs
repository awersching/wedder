use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::config::cmd_args::CmdArgs;
use crate::location::Location;
use crate::location::LocationProvider;
use crate::weather::providers::WeatherProvider;

pub mod cmd_args;

pub const RETRY_TIMEOUT: u64 = 5;

const APP_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub format: String,
    pub interval: i32,
    pub weather: WeatherConfig,
    pub location: LocationConfig,
    pub icons: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherConfig {
    pub provider: WeatherProvider,
    pub api_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationConfig {
    pub provider: LocationProvider,
    #[serde(flatten)]
    pub location: Location,
}

impl Config {
    pub fn from_default_path() -> Self {
        let config_path = Config::default_config_path();
        Config::load_config(&config_path)
    }

    pub fn from_path(path: &str) -> Config {
        let config_path = [path].iter().collect();
        Config::load_config(&config_path)
    }

    pub fn merge(&mut self, args: CmdArgs) {
        if let Some(format) = args.format {
            self.format = format;
        }
        if let Some(interval) = args.interval {
            self.interval = interval;
        }

        if let Some(weather_provider) = args.weather_provider {
            self.weather.provider = weather_provider;
        }
        if let Some(weather_api_key) = args.weather_api_key {
            self.weather.api_key = weather_api_key;
        }

        if let Some(location_provider) = args.location_provider {
            self.location.provider = location_provider;
        }
        if let Some(lat) = args.lat {
            self.location.location.lat = lat;
        }
        if let Some(lon) = args.lon {
            self.location.location.lon = lon;
        }
    }

    pub fn default_config_path() -> PathBuf {
        let project = match
            ProjectDirs::from("rs", APP_NAME, APP_NAME) {
            Some(dir) => dir,
            None => Config::no_config_file()
        };

        [
            project.config_dir().to_str().unwrap(),
            &format!("{}.toml", APP_NAME),
        ].iter().collect()
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

    fn no_config_file() -> ! {
        println!("No config file");
        process::exit(1)
    }

    fn malformed_config() -> ! {
        println!("Malformed config file");
        process::exit(1)
    }
}
