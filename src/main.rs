use std::thread;
use std::time;

use log::error;

use crate::config::Config;
use crate::location::CurrentLocation;
use crate::location::ip_api::IpApi;
use crate::weather::CurrentWeather;
use crate::weather::providers::owm::OpenWeatherMap;

mod config;
mod weather;
mod util;
mod location;

pub fn main() {
    let config = Config::new();
    let location_provider = IpApi::new();
    let weather_provider = OpenWeatherMap::new(config.weather_api_key);

    loop {
        let current_location = match location_provider.current_location() {
            Ok(location) => location,
            Err(err) => {
                error!("{:?}", err);
                thread::sleep(time::Duration::from_secs(config::RETRY_TIMEOUT));
                continue;
            }
        };

        match weather_provider.current_weather(&current_location.city.unwrap()) {
            Ok(weather) => println!("{}", weather.format(&config.format, &config.icons)),
            Err(err) => error!("{:?}", err)
        }
        thread::sleep(time::Duration::from_secs(config.interval as u64));
    }
}
