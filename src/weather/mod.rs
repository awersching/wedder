use crate::weather::weather_status::WeatherStatus;

pub mod providers;
pub mod weather_status;
pub mod weather_condition;

pub trait Weather {
    fn new(api_key: String) -> Self;

    fn current_weather(&self, location: String) -> WeatherStatus;
}
