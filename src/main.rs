use std::thread;
use std::time;

use log::error;

use crate::config::Config;
use crate::location::CurrentLocation;
use crate::location::ip_api::IpApi;
use crate::location::Location;
use crate::location::LocationProvider;
use crate::weather::CurrentWeather;
use crate::weather::providers::owm::OpenWeatherMap;

mod config;
mod weather;
mod util;
mod location;

pub fn main() {
    let config = Config::new();
    let location_provider = IpApi::new();
    let weather_provider = OpenWeatherMap::new(&config.weather.api_key);

    loop {
        let current_location = match current_location(&location_provider, &config) {
            Some(location) => location,
            None => continue
        };

        match weather_provider.current_weather(&current_location) {
            Ok(weather) => println!("{}", weather.format(&config.format, &config.icons)),
            Err(err) => error!("{:?}", err)
        }
        thread::sleep(time::Duration::from_secs(config.interval as u64));
    }
}

fn current_location(provider: &impl CurrentLocation, config: &Config) -> Option<Location> {
    match config.location.provider {
        LocationProvider::Ip => match provider.current_location() {
            Ok(location) => Some(location),
            Err(err) => {
                error!("{:?}", err);
                thread::sleep(time::Duration::from_secs(config::RETRY_TIMEOUT));
                None
            }
        },

        LocationProvider::Manual => Some(config.location.location.clone())
    }
}
