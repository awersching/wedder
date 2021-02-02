use std::num::ParseIntError;
use std::str::FromStr;
use std::string::ParseError;
use std::{env, process};

use log::debug;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use strum_macros::EnumString;

use crate::config::cli_args::CliArgs;
use crate::location::Location;
use crate::location::LocationProvider;
use crate::weather::weather_condition::Icons;
use crate::weather::WeatherProvider;

mod cli_args;
mod file;

const WEDDER_WEATHER_API_KEY: &str = "WEDDER_WEATHER_API_KEY";

macro_rules! merge {
    ($config_value:expr, $args_value:expr) => {
        if let Some(x) = $args_value {
            $config_value = x
        }
    };
}

#[derive(Default, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Config {
    #[serde(default)]
    pub format: Format,
    #[serde(default)]
    pub interval: Interval,
    #[serde(default)]
    pub units: Units,
    #[serde(default)]
    pub weather: WeatherConfig,
    #[serde(default)]
    pub location: LocationConfig,
    #[serde(default)]
    pub icons: Icons,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Format(pub String);

impl Default for Format {
    fn default() -> Self {
        Self("<icon> <temperature>°C".to_string())
    }
}

impl FromStr for Format {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Interval(pub i32);

impl Default for Interval {
    fn default() -> Self {
        Self(300)
    }
}

impl FromStr for Interval {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        i32::from_str(s).map(Self)
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Units {
    #[serde(default)]
    pub temperature: Temperature,
    #[serde(default)]
    pub wind_speed: WindSpeed,
    #[serde(default)]
    pub distance: Distance,
    #[serde(default)]
    pub precipitation: Precipitation,
}

#[derive(Debug, Serialize, Deserialize, EnumString, Eq, PartialEq, Clone)]
pub enum Temperature {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl Default for Temperature {
    fn default() -> Self {
        Self::Celsius
    }
}

#[derive(Debug, Serialize, Deserialize, EnumString, Eq, PartialEq, Clone)]
pub enum WindSpeed {
    Ms,
    Kmh,
    Mph,
}

impl Default for WindSpeed {
    fn default() -> Self {
        Self::Kmh
    }
}

#[derive(Debug, Serialize, Deserialize, EnumString, Eq, PartialEq, Clone)]
pub enum Distance {
    Meter,
    Kilometer,
    Mile,
}

impl Default for Distance {
    fn default() -> Self {
        Self::Kilometer
    }
}

#[derive(Debug, Serialize, Deserialize, EnumString, Eq, PartialEq, Clone)]
pub enum Precipitation {
    Millimeter,
    Inch,
}

impl Default for Precipitation {
    fn default() -> Self {
        Self::Millimeter
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct WeatherConfig {
    #[serde(default)]
    pub provider: WeatherProvider,
    #[serde(default)]
    pub api_key: String,
}

#[derive(Default, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct LocationConfig {
    #[serde(default)]
    pub provider: LocationProvider,
    #[serde(default, flatten)]
    pub location: Location,
}

impl Config {
    pub fn new() -> Self {
        let args = CliArgs::from_args();
        args.apply();

        let mut config = match &args.config_file {
            Some(path) => file::from_path(&[path].iter().collect()),
            None => file::from_default_path(),
        };
        debug!("Read {:#?}", config);
        config.merge(args);
        debug!("Merged config with args into {:#?}", config);

        if config.weather.api_key.is_empty() {
            if let Ok(key) = env::var(WEDDER_WEATHER_API_KEY) {
                config.weather.api_key = key;
            } else {
                println!("No API key");
                process::exit(1)
            }
        }
        config
    }

    fn merge(&mut self, args: CliArgs) {
        merge!(self.format, args.format);
        merge!(self.interval, args.interval);
        merge!(self.units.temperature, args.temperature_unit);
        merge!(self.units.wind_speed, args.wind_speed_unit);
        merge!(self.units.distance, args.distance_unit);
        merge!(self.units.precipitation, args.precipitation_unit);
        merge!(self.weather.provider, args.weather_provider);
        merge!(self.weather.api_key, args.weather_api_key);
        merge!(self.location.provider, args.location_provider);
        merge!(self.location.location.lat, args.lat);
        merge!(self.location.location.lon, args.lon);
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::config::cli_args::CliArgs;
    use crate::config::Distance::Mile;
    use crate::config::Precipitation::Inch;
    use crate::config::Temperature::Kelvin;
    use crate::config::WindSpeed::Ms;
    use crate::config::{Config, Format, Interval};
    use crate::location::LocationProvider::Manual;
    use crate::weather::WeatherProvider::OwmMock;

    #[test]
    fn default() {
        let cfg_str = fs::read_to_string("examples/wedder.toml").unwrap();
        let file: Config = toml::from_str(&cfg_str).unwrap();
        let default = Config::default();

        assert_eq!(file.format, default.format);
        assert_eq!(file.interval, default.interval);
        assert_eq!(file.units, default.units);
        assert_eq!(file.weather, default.weather);
        assert_eq!(file.location, default.location);
        assert_eq!(file.icons, default.icons);
    }

    #[test]
    fn merge() {
        let args = CliArgs {
            debug: false,
            default_config_path: false,
            config_file: None,
            format: Some(Format("format".to_string())),
            interval: Some(Interval(123)),
            temperature_unit: Some(Kelvin),
            wind_speed_unit: Some(Ms),
            distance_unit: Some(Mile),
            precipitation_unit: Some(Inch),
            weather_provider: Some(OwmMock),
            weather_api_key: Some("key".to_string()),
            location_provider: Some(Manual),
            lat: Some(1.0),
            lon: Some(1.0),
        };
        let mut config = Config::default();
        config.merge(args.clone());

        assert_eq!(config.format, args.format.unwrap());
        assert_eq!(config.interval, args.interval.unwrap());
        assert_eq!(config.units.temperature, Kelvin);
        assert_eq!(config.units.wind_speed, Ms);
        assert_eq!(config.units.distance, Mile);
        assert_eq!(config.units.precipitation, Inch);
        assert_eq!(config.weather.provider, args.weather_provider.unwrap());
        assert_eq!(config.weather.api_key, args.weather_api_key.unwrap());
        assert_eq!(config.location.provider, args.location_provider.unwrap());
        assert_eq!(config.location.location.lat, args.lat.unwrap());
        assert_eq!(config.location.location.lon, args.lon.unwrap());
    }
}
