use crate::settings::Settings;
use crate::weather::weather_status::WeatherStatus;

pub mod owm;
pub mod weather_status;

pub trait Weather {
    fn new(settings: &Settings) -> Self;

    fn current_weather(&self) -> WeatherStatus;
}
