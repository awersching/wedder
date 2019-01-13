use std::error::Error;
use std::thread;
use std::time;

use log::error;

use crate::config::Config;
use crate::location::CurrentLocation;
use crate::location::ip_api::IpApi;
use crate::location::Location;
use crate::location::LocationProvider;
use crate::weather::providers::CurrentWeather;
use crate::weather::providers::owm::OpenWeatherMap;

mod config;
mod weather;
mod util;
mod location;

struct App {
    config: Box<Config>,
    location_provider: Box<dyn CurrentLocation>,
    weather_provider: Box<dyn CurrentWeather>,
}

impl App {
    fn new() -> Self {
        App {
            config: Box::new(Config::new()),
            location_provider: Box::new(IpApi::new()),
            weather_provider: Box::new(OpenWeatherMap::new()),
        }
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        loop {
            if self.config.location.provider == LocationProvider::Manual {
                self.print_current_weather(&self.config.location.location)?;
            } else {
                // location by provider
                let current_location = self.location_provider.current_location()?;
                self.print_current_weather(&current_location)?;
            }
            thread::sleep(time::Duration::from_secs(self.config.interval as u64));
        }
    }

    fn print_current_weather(&self, location: &Location) -> Result<(), Box<dyn Error>> {
        let current_weather = self.weather_provider
            .current_weather(location, &self.config.weather.api_key)?;
        println!("{}", current_weather.format(&self.config.format, &self.config.icons));
        Ok(())
    }
}

fn main() {
    let app = App::new();

    if let Err(err) = app.run() {
        error!("{:?}", err)
    }
}
