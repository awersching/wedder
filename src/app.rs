use std::{process, thread};
use std::time;

use log::debug;

use crate::config::Config;
use crate::location::{CurrentLocation, Location};
use crate::location::LocationProvider;
use crate::Result;
use crate::weather::Formatter;
use crate::weather::providers::{CurrentWeather, WeatherProvider};

pub struct App {
    config: Config,
    location_provider: Box<dyn CurrentLocation>,
    weather_provider: Box<dyn CurrentWeather>,
}

impl App {
    pub fn new(config: Config) -> Self {
        let location_provider =
            LocationProvider::create(&config.location);
        let weather_provider =
            WeatherProvider::create(&config.weather.provider);

        Self {
            config,
            location_provider,
            weather_provider,
        }
    }

    pub fn run(&self) -> Result<()> {
        loop {
            debug!("Pulling current location...");
            let location = self.location_provider.location()?;
            debug!("{:?}", location);
            let weather = self.weather(&location)?;
            println!("{}", weather);

            self.sleep();
        }
    }

    fn weather(&self, location: &Location) -> Result<String> {
        debug!("Pulling current weather...");
        let weather = self.weather_provider
            .weather(location, &self.config.weather.api_key)?;

        let formatted = Formatter::new(
            &self.config.format,
            weather,
            &self.config.icons,
        ).format();
        Ok(formatted)
    }

    fn sleep(&self) {
        if let Some(interval) = self.config.interval {
            debug!("Sleeping for {}s...", interval);
            thread::sleep(time::Duration::from_secs(interval));
        } else {
            debug!("Exiting because no interval for loop is specified...");
            process::exit(0);
        }
    }
}
