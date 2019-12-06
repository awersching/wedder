use serde::Deserialize;
use serde::Serialize;
use strum_macros::EnumString;

use crate::location::Location;
use crate::Result;
use crate::weather::providers::owm::OpenWeatherMap;
use crate::weather::providers::owm::tests::OwmMock;
use crate::weather::Weather;

pub mod owm;

pub trait CurrentWeather {
    fn weather(&self, location: &Location, api_key: &str) -> Result<Box<dyn Weather>>;
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
            Self::OwmMock => Box::new(OwmMock::new())
        }
    }
}

impl Default for WeatherProvider {
    fn default() -> Self {
        Self::OpenWeatherMap
    }
}
