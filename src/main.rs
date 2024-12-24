use std::error::Error;
use std::{process, thread, time};

use adapters::location::ip_api::IpApi;
use adapters::location::manual::Manual;
use adapters::weather::owm::OpenWeatherMap;
use log::debug;
use model::config::{Config, LocationConfig};
use model::location::{CurrentLocation, Location, LocationProvider};
use model::weather::{CurrentWeather, WeatherProvider};
use services::config_service;
use services::format_service::FormatService;

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
        let current_location = Self::current_location(&config.location);
        let current_weather = Self::current_weather(&config.weather.provider);

        Self {
            config,
            current_location,
            current_weather,
        }
    }

    fn current_location(location_config: &LocationConfig) -> Box<dyn CurrentLocation> {
        match &location_config.provider {
            LocationProvider::IpApi => Box::new(IpApi::new()),
            LocationProvider::Manual => Box::new(Manual::new(&location_config.location)),
        }
    }

    fn current_weather(provider: &WeatherProvider) -> Box<dyn CurrentWeather> {
        match provider {
            WeatherProvider::OpenWeatherMap => Box::new(OpenWeatherMap::new()),
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
