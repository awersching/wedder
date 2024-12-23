use std::error::Error;
use std::{process, thread, time};

use log::debug;
use model::config::Config;
use model::location::{CurrentLocation, Location};
use model::weather::CurrentWeather;
use services::format_service::FormatService;
use services::{config_service, location_service, weather_service};

mod adapters;
mod logger;
mod model;
mod services;

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

const APP_NAME: &str = env!("CARGO_PKG_NAME");

fn main() {
    if let Err(err) = run() {
        println!("Error: {}", err);
        process::exit(1);
    }
}

fn run() -> crate::Result<()> {
    let config = config_service::config();
    let app = App::new(config);
    app.run()
}

struct App {
    config: Config,
    current_location: Box<dyn CurrentLocation>,
    current_weather: Box<dyn CurrentWeather>,
}

impl App {
    fn new(config: Config) -> Self {
        let current_location = location_service::current_location(&config.location);
        let current_weather = weather_service::current_weather(&config.weather.provider);

        Self {
            config,
            current_location,
            current_weather,
        }
    }

    fn run(&self) -> Result<()> {
        loop {
            debug!("Polling current location...");
            let location = self.current_location.location()?;
            debug!("{:#?}", location);
            debug!("Polling current weather...");
            let weather = self.weather(location)?;
            println!("{}", weather);

            self.sleep();
        }
    }

    fn weather(&self, location: Location) -> Result<String> {
        let weather = self
            .current_weather
            .weather(&location, &self.config.weather.api_key)?;
        let formatted = FormatService::new(&self.config, location, weather).format();
        Ok(formatted)
    }

    fn sleep(&self) {
        if self.config.interval.0 >= 0 {
            debug!("Sleeping for {}s...", self.config.interval.0);
            thread::sleep(time::Duration::from_secs(self.config.interval.0 as u64));
        } else {
            debug!("Exiting because of negative interval...");
            process::exit(0);
        }
    }
}
