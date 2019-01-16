use serde::Deserialize;
use serde::Serialize;

use crate::location::Location;
use crate::util;
use crate::weather::Weather;

pub mod owm;

pub trait CurrentWeather {
    fn current_weather(&self, location: &Location, api_key: &str) -> util::Result<Weather>;
}

#[derive(Debug, Serialize, Deserialize, EnumString)]
pub enum WeatherProvider {
    OpenWeatherMap
}

impl Default for WeatherProvider {
    fn default() -> Self {
        WeatherProvider::OpenWeatherMap
    }
}
