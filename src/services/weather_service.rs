use crate::{
    adapters::weather::owm::OpenWeatherMap,
    model::weather::{CurrentWeather, WeatherProvider},
};

pub fn current_weather(provider: &WeatherProvider) -> Box<dyn CurrentWeather> {
    match provider {
        WeatherProvider::OpenWeatherMap => Box::new(OpenWeatherMap::new()),
    }
}
