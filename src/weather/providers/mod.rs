use serde::Deserialize;
use serde::Serialize;

pub mod owm;

#[derive(Serialize, Deserialize)]
pub enum WeatherProvider {
    OpenWeatherMap
}
