use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

use crate::location::Location;
use crate::weather::owm::{OpenWeatherMap, OwmMock};
use crate::weather::weather_condition::WeatherCondition;
use crate::Result;

pub mod formatter;
mod owm;
pub mod weather_condition;

pub trait CurrentWeather {
    fn weather(&self, location: &Location, api_key: &str) -> Result<Box<dyn Weather>>;
}

pub trait Weather {
    fn weather_condition(&self) -> WeatherCondition;

    fn kelvin(&self) -> f32;
    fn kelvin_feels_like(&self) -> f32;
    fn kelvin_max(&self) -> f32;
    fn kelvin_min(&self) -> f32;

    fn pressure(&self) -> f32;
    fn humidity(&self) -> f32;
    fn wind_speed(&self) -> f32;
    fn cloud_percentage(&self) -> f32;

    fn sunrise(&self) -> DateTime<Local>;
    fn sunset(&self) -> DateTime<Local>;
}

#[derive(Debug, Serialize, Deserialize, EnumString, Eq, PartialEq, Clone)]
pub enum WeatherProvider {
    OpenWeatherMap,
    OwmMock,
}

impl WeatherProvider {
    pub fn create(provider: &Self) -> Box<dyn CurrentWeather> {
        match provider {
            Self::OpenWeatherMap => Box::new(OpenWeatherMap::new()),
            Self::OwmMock => Box::new(OwmMock::new()),
        }
    }
}

impl Default for WeatherProvider {
    fn default() -> Self {
        Self::OpenWeatherMap
    }
}
