use std::thread;
use std::time;

use log::debug;

use crate::config::Config;
use crate::location::CurrentLocation;
use crate::location::ip_api::IpApi;
use crate::location::Location;
use crate::location::LocationProvider;
use crate::util;
use crate::weather::providers::CurrentWeather;
use crate::weather::providers::owm::OpenWeatherMap;

pub struct App {
    config: Config,
    location_provider: Box<dyn CurrentLocation>,
    weather_provider: Box<dyn CurrentWeather>,
}

impl App {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            location_provider: Box::new(IpApi::new()),
            weather_provider: Box::new(OpenWeatherMap::new()),
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
            thread::sleep(time::Duration::from_secs(self.config.interval));
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
