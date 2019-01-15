use std::thread;
use std::time;

use log::debug;

use crate::config::Config;
use crate::location::CurrentLocation;
use crate::location::Location;
use crate::location::LocationProvider;
use crate::util;
use crate::weather::providers::CurrentWeather;

pub struct App<T: CurrentLocation, U: CurrentWeather> {
    config: Config,
    location_provider: T,
    weather_provider: U,
}

impl<T: CurrentLocation, U: CurrentWeather> App<T, U> {
    pub fn new(config: Config, location_provider: T, weather_provider: U) -> Self {
        App {
            config,
            location_provider,
            weather_provider,
        }
    }

    pub fn run(&self) -> util::Result<()> {
        loop {
            if self.config.location.provider == LocationProvider::Manual {
                debug!("Using location from config file");
                self.print_current_weather(&self.config.location.location)?;
            } else {
                debug!("Pulling current location...");
                let current_location = self.location_provider.current_location()?;
                self.print_current_weather(&current_location)?;
            }

            debug!("Sleeping for {}s...", self.config.interval.to_string());
            thread::sleep(time::Duration::from_secs(self.config.interval as u64));
        }
    }

    fn print_current_weather(&self, location: &Location) -> util::Result<()> {
        debug!("{:?}", location);
        debug!("Pulling current weather...");
        let current_weather = self.weather_provider
            .current_weather(location, &self.config.weather.api_key)?;
        debug!("{:?}", current_weather);

        println!("{}", current_weather.format(&self.config.format, &self.config.icons));
        Ok(())
    }
}
