extern crate confy;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::thread;
use std::time;

use crate::config::Config;
use crate::weather::CurrentWeather;
use crate::weather::providers::owm::OpenWeatherMap;

mod config;
mod weather;
mod location;

pub fn run() {
    let config = Config::new();
    let weather = OpenWeatherMap::new(config.weather_api_key);

    loop {
        println!("{}", weather.current_weather(&config.city).to_string());
        thread::sleep(time::Duration::from_secs(config.interval as u64));
    }
}
