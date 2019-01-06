extern crate config;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use crate::settings::Settings;
use crate::weather::providers::owm::OpenWeatherMap;
use crate::weather::Weather;

mod settings;
mod weather;

pub fn run() {
    let settings = Settings::new();
    let weather = OpenWeatherMap::new(settings.weather_api_key);

    println!("{}", weather.current_weather(settings.city).to_string());
}
