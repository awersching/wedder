extern crate config;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::thread;
use std::time;

use crate::settings::Settings;
use crate::weather::providers::owm::OpenWeatherMap;
use crate::weather::Weather;

mod settings;
mod weather;

pub fn run() {
    let settings = Settings::new();
    let weather = OpenWeatherMap::new(&settings);

    loop {
        println!("{}", weather.current_weather().to_string());
        thread::sleep(time::Duration::from_secs(settings.interval as u64));
    }
}
