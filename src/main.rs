use std::error::Error;
use std::{process, thread, time};

use log::debug;

use crate::config::Config;
use crate::location::{CurrentLocation, Location, LocationProvider};
use crate::weather::formatter::Formatter;
use crate::weather::{CurrentWeather, WeatherProvider};

mod config;
mod http;
mod location;
mod logger;
mod weather;

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

const APP_NAME: &str = env!("CARGO_PKG_NAME");

fn main() {
    if let Err(err) = run() {
        println!("Error: {}", err);
        process::exit(1);
    }
}

fn run() -> crate::Result<()> {
    let config = Config::new();
    let app = App::new(config);
    app.run()
}

struct App {
    config: Config,
    location_provider: Box<dyn CurrentLocation>,
    weather_provider: Box<dyn CurrentWeather>,
}

impl App {
    fn new(config: Config) -> Self {
        let location_provider = LocationProvider::create(&config.location);
        let weather_provider = WeatherProvider::create(&config.weather.provider);

        Self {
            config,
            location_provider,
            weather_provider,
        }
    }

    fn run(&self) -> Result<()> {
        loop {
            debug!("Pulling current location...");
            let location = self.location_provider.location()?;
            debug!("{:#?}", location);
            debug!("Pulling current weather...");
            let weather = self.weather(location)?;
            println!("{}", weather);

            self.sleep();
        }
    }

    fn weather(&self, location: Location) -> Result<String> {
        let weather = self
            .weather_provider
            .weather(&location, &self.config.weather.api_key)?;
        let formatted = Formatter::new(&self.config, location, weather).format();
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
