use std::collections::HashMap;
use std::path::PathBuf;
use std::process;

use log::debug;
use serde::{Deserialize, Serialize};

use crate::config::cli_args::CliArgs;
use crate::location::Location;
use crate::location::LocationProvider;
use crate::weather::providers::WeatherProvider;
use crate::weather::weather_condition;

pub mod cli_args;
mod file;

pub const RETRY_TIMEOUT: u64 = 15;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_format")]
    pub format: String,
    #[serde(default = "default_interval")]
    pub interval: u64,
    #[serde(default)]
    pub weather: WeatherConfig,
    #[serde(default)]
    pub location: LocationConfig,
    #[serde(default = "weather_condition::default_icons")]
    pub icons: HashMap<String, String>,
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
    pub fn new(args: CliArgs) -> Self {
        let mut config = match &args.config_file {
            Some(path) => Self::from_path(&[path].iter().collect()),
            None => Self::from_default_path()
        };
        debug!("Read {:?}", config);
        config.merge(args);
        debug!("Merged config with args into {:?}", config);

        if config.weather.api_key == "" {
            println!("No API key");
            process::exit(1)
        }
        config
    }

    fn from_default_path() -> Self {
        let default_path = if let Some(path) = file::default_config_path() {
            path
        } else {
            println!("Erroneous default config path");
            process::exit(1)
        };
        Self::from_path(&default_path)
    }

    fn from_path(path: &PathBuf) -> Self {
        if let Some(config) = file::load_config(&path) {
            config
        } else {
            println!("Erroneous config path");
            process::exit(1)
        }
    }

    fn merge(&mut self, args: CliArgs) {
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
}

impl Default for Config {
    fn default() -> Self {
        Self {
            format: default_format(),
            interval: default_interval(),
            weather: WeatherConfig::default(),
            location: LocationConfig::default(),
            icons: weather_condition::default_icons(),
        }
    }
}

/// Remove when serde supports default literals
/// https://github.com/serde-rs/serde/issues/368
fn default_format() -> String {
    "<icon> <temperature_celsius>°C".to_string()
}

/// Remove when serde supports default literals
/// https://github.com/serde-rs/serde/issues/368
fn default_interval() -> u64 {
    300
}
