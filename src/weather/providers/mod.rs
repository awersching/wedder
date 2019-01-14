use std::error::Error;

use serde::Deserialize;
use serde::Serialize;

use crate::location::Location;
use crate::weather::Weather;

pub mod owm;

pub trait CurrentWeather {
    fn current_weather(&self, location: &Location, api_key: &str)
                       -> Result<Weather, Box<dyn Error>>;
}

#[derive(Debug, Serialize, Deserialize, EnumString)]
pub enum WeatherProvider {
    OpenWeatherMap
}
