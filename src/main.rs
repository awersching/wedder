use std::{process, thread, time};
use std::error::Error;

use log::debug;
use structopt::StructOpt;

use crate::config::cli_args::CliArgs;
use crate::config::Config;
use crate::location::{CurrentLocation, Location, LocationProvider};
use crate::weather::Formatter;
use crate::weather::providers::{CurrentWeather, WeatherProvider};

mod config;
mod weather;
mod location;
mod http;
mod logger;

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

const APP_NAME: &str = env!("CARGO_PKG_NAME");

fn main() {
    if let Err(err) = run() {
        println!("Error: {}", err.to_string());
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = CliArgs::from_args();
    args.apply();
    let config = Config::new(args);

    let app = App::new(config);
    app.run()
}

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
