use std::collections::HashMap;
use std::fmt::Display;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process;

use directories::ProjectDirs;
use log::debug;
use log::error;
use log::warn;
use serde::{Deserialize, Serialize};

use crate::config::cmd_args::CmdArgs;
use crate::location::Location;
use crate::location::LocationProvider;
use crate::weather::providers::WeatherProvider;
use crate::weather::weather_condition;

pub mod cmd_args;

pub const RETRY_TIMEOUT: u64 = 5;

const APP_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_format")]
    pub format: String,
    #[serde(default = "default_interval")]
    pub interval: i32,
    #[serde(default)]
    pub weather: WeatherConfig,
    #[serde(default)]
    pub location: LocationConfig,
    #[serde(default = "weather_condition::default_icons")]
    pub icons: HashMap<String, String>,
}

/// Remove when serde supports default literals
fn default_format() -> String {
    "<icon> <temperature_celsius>°C".to_string()
}

/// Remove when serde supports default literals
fn default_interval() -> i32 {
    300
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct WeatherConfig {
    #[serde(default)]
    pub provider: WeatherProvider,
    #[serde(default)]
    pub api_key: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct LocationConfig {
    #[serde(default)]
    pub provider: LocationProvider,
    #[serde(default, flatten)]
    pub location: Location,
}

impl Config {
    pub fn from_default_path() -> Self {
        let config_path = Config::default_config_path();
        Config::load_config(&config_path.unwrap())
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

    pub fn default_config_path() -> Option<PathBuf> {
        let project =
            ProjectDirs::from("rs", APP_NAME, APP_NAME)?;

        Some([
            project.config_dir().to_str().unwrap(),
            &format!("{}.toml", APP_NAME),
        ].iter().collect())
    }

    fn load_config(path: &PathBuf) -> Config {
        debug!("Trying to open config file under {}", path.to_str().unwrap());
        let config_string = match fs::read_to_string(path) {
            Ok(cfg_str) => Some(cfg_str),
            Err(err) => match err.kind() {
                io::ErrorKind::NotFound => None,
                _ => Config::malformed_config(err)
            }
        };

        if config_string.is_none() {
            warn!(
                "No config file found under {}, using defaults",
                path.to_str().unwrap()
            );
            return Config::default();
        }

        match toml::from_str(&config_string.unwrap()) {
            Ok(config) => config,
            Err(err) => Config::malformed_config(err)
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    fn malformed_config<E: Display>(err: E) -> ! {
        error!("Error parsing config file: {}", err.to_string());
        println!("Malformed config file");
        process::exit(1)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            format: default_format(),
            interval: default_interval(),
            weather: WeatherConfig::default(),
            location: LocationConfig::default(),
            icons: weather_condition::default_icons(),
        }
    }
}
