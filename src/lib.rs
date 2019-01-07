extern crate confy;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use crate::config::Config;
use crate::weather::CurrentWeather;
use crate::weather::providers::owm::OpenWeatherMap;

mod config;
mod weather;
mod location;

pub fn run() {
    let config = Config::new();
    let weather = OpenWeatherMap::new(config.weather_api_key);

    println!("{}", weather.current_weather(config.city).to_string());
}
