use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

use crate::model::config::{PrecipitationUnit, WindSpeedUnit};
use crate::model::location::Location;
use std::fmt;
use std::fmt::{Display, Formatter};

use strum_macros::Display;

use super::config::{DistanceUnit, TemperatureUnit};

pub trait CurrentWeather {
    fn weather(&self, location: &Location, api_key: &str) -> crate::Result<Box<dyn Weather>>;
}

pub trait Weather {
    fn weather_condition(&self) -> Option<WeatherCondition>;

    fn temp(&self) -> Option<Kelvin>;
    fn temp_feels_like(&self) -> Option<Kelvin>;
    fn temp_max(&self) -> Option<Kelvin>;
    fn temp_min(&self) -> Option<Kelvin>;
    fn dew_point(&self) -> Option<Kelvin>;

    fn precipitation(&self) -> Option<Millimeter>;
    fn precipitation_chance(&self) -> Option<Percentage>;
    fn clouds(&self) -> Option<Percentage>;
    fn humidity(&self) -> Option<Percentage>;
    fn visibility(&self) -> Option<Meter>;
    fn wind_speed(&self) -> Option<Ms>;
    fn pressure(&self) -> Option<Hpa>;
    fn uvi(&self) -> Option<Uvi>;
    fn aqi(&self) -> Option<Aqi>;

    fn sunrise(&self) -> Option<DateTime<Local>>;
    fn sunset(&self) -> Option<DateTime<Local>>;
}

pub trait Convert {
    type Unit;
    fn convert(&self, unit: &Self::Unit) -> String;
}

#[derive(Debug, Deserialize, Copy, Clone, PartialEq)]
pub struct Kelvin(pub f32);

impl Convert for Kelvin {
    type Unit = TemperatureUnit;
    fn convert(&self, unit: &Self::Unit) -> String {
        let converted = match unit {
            TemperatureUnit::Celsius => self.0 - 273.15,
            TemperatureUnit::Fahrenheit => (self.0 - 273.15) * (9.0 / 5.0) + 32.0,
            TemperatureUnit::Kelvin => self.0,
        };
        format!("{:.0}", converted)
    }
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct Millimeter(pub f32);

impl Convert for Millimeter {
    type Unit = PrecipitationUnit;
    fn convert(&self, unit: &Self::Unit) -> String {
        let converted = match unit {
            PrecipitationUnit::Millimeter => self.0,
            PrecipitationUnit::Inch => self.0 / 25.4,
        };
        format!("{:.3}", converted)
    }
}

#[derive(Debug, Deserialize, Copy, Clone, PartialEq)]
pub struct Percentage(pub f32);

impl Display for Percentage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:.0}", self.0)
    }
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct Meter(pub f32);

impl Convert for Meter {
    type Unit = DistanceUnit;
    fn convert(&self, unit: &Self::Unit) -> String {
        let converted = match unit {
            DistanceUnit::Meter => self.0,
            DistanceUnit::Kilometer => self.0 / 1000.0,
            DistanceUnit::Mile => self.0 * 0.000_621_371_2,
        };
        format!("{:.1}", converted)
    }
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct Ms(pub f32);

impl Convert for Ms {
    type Unit = WindSpeedUnit;
    fn convert(&self, unit: &Self::Unit) -> String {
        let converted = match unit {
            WindSpeedUnit::Ms => self.0,
            WindSpeedUnit::Kmh => self.0 * 3.6,
            WindSpeedUnit::Mph => self.0 * (3600.0 / 1609.34),
        };
        format!("{:.1}", converted)
    }
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct Hpa(pub f32);

impl Display for Hpa {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:.0}", self.0)
    }
}

#[derive(Debug, Deserialize, Copy, Clone, PartialEq)]
pub struct Uvi(pub f32);

impl Display for Uvi {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:.0}", self.0)
    }
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct Aqi(pub f32);

impl Display for Aqi {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:.0}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize, EnumString, Eq, PartialEq, Clone)]
pub enum WeatherProvider {
    OpenWeatherMap,
}

impl Default for WeatherProvider {
    fn default() -> Self {
        Self::OpenWeatherMap
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Serialize, Deserialize, Display)]
#[strum(serialize_all = "snake_case")]
pub enum WeatherCondition {
    ClearSky,
    FewClouds,
    Clouds,
    ManyClouds,
    Rain,
    HeavyRain,
    Thunderstorm,
    Snow,
    Mist,
}
