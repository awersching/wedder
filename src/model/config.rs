use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;
use std::string::ParseError;
use strum_macros::EnumString;

use crate::model::location::{Location, LocationProvider};
use crate::model::weather::WeatherProvider;

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
    pub temperature: TemperatureUnit,
    #[serde(default)]
    pub wind_speed: WindSpeedUnit,
    #[serde(default)]
    pub distance: DistanceUnit,
    #[serde(default)]
    pub precipitation: PrecipitationUnit,
}

#[derive(Debug, Serialize, Deserialize, EnumString, Eq, PartialEq, Clone)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl Default for TemperatureUnit {
    fn default() -> Self {
        Self::Celsius
    }
}

#[derive(Debug, Serialize, Deserialize, EnumString, Eq, PartialEq, Clone)]
pub enum WindSpeedUnit {
    Ms,
    Kmh,
    Mph,
}

impl Default for WindSpeedUnit {
    fn default() -> Self {
        Self::Kmh
    }
}

#[derive(Debug, Serialize, Deserialize, EnumString, Eq, PartialEq, Clone)]
pub enum DistanceUnit {
    Meter,
    Kilometer,
    Mile,
}

impl Default for DistanceUnit {
    fn default() -> Self {
        Self::Kilometer
    }
}

#[derive(Debug, Serialize, Deserialize, EnumString, Eq, PartialEq, Clone)]
pub enum PrecipitationUnit {
    Millimeter,
    Inch,
}

impl Default for PrecipitationUnit {
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

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Icons(HashMap<String, String>);

impl Icons {
    pub fn get(&self, condition: &str) -> Option<&String> {
        self.0.get(condition)
    }
}

impl Default for Icons {
    fn default() -> Self {
        let cfg_str = include_str!("../../examples/wedder.toml");
        let config: Config = toml::from_str(cfg_str).unwrap();
        config.icons
    }
}
