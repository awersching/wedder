use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use serde::{Deserialize, Serialize};

use crate::weather::weather::Weather;

pub mod providers;
#[allow(clippy::module_inception)]
pub mod weather;

pub trait CurrentWeather {
    fn new(api_key: String) -> Self;

    fn current_weather(&self, location: &str) -> Result<Weather, Box<dyn Error>>;
}

#[derive(Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum WeatherCondition {
    ClearSky,
    FewClouds,
    Clouds,
    ManyClouds,
    RainSun,
    Rain,
    HeavyRain,
    Thunderstorm,
    Snow,
    Mist,
}

impl Display for WeatherCondition {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
