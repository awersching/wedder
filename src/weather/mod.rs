use crate::weather::weather::Weather;

pub mod providers;
pub mod weather;
pub mod weather_condition;

pub trait CurrentWeather {
    fn new(api_key: String) -> Self;

    fn current_weather(&self, location: String) -> Weather;
}
