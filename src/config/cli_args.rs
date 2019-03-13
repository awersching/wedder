use std::process;

use log::debug;
use log::error;
use structopt::StructOpt;

use crate::config;
use crate::location::CurrentLocation;
use crate::location::ip_api::IpApi;
use crate::location::LocationProvider;
use crate::logger;
use crate::util::Result;
use crate::weather::providers::WeatherProvider;

#[derive(Debug, StructOpt)]
#[structopt()]
pub struct CliArgs {
    /// Enables verbose debug output
    #[structopt(short, long)]
    pub debug: bool,

    /// Prints the default config path
    #[structopt(short = "p", long = "default-config-path")]
    pub default_config_path: bool,
    /// Path to an alternative config file
    #[structopt(short, long = "config-file")]
    pub config_file: Option<String>,

    /// The format to display the weather status in
    ///
    /// Available tags:
    /// <icon>,
    /// <temperature_kelvin>,
    /// <kelvin_max>,
    /// <kelvin_min>,
    /// <temperature_celsius>,
    /// <celsius_max>,
    /// <celsius_min>,
    /// <temperature_fahrenheit>,
    /// <fahrenheit_max>,
    /// <fahrenheit_min>,
    /// <pressure>,
    /// <humidity>,
    /// <wind_speed>,
    /// <cloud_percentage>,
    /// <sunrise>,
    /// <sunset>
    ///
    /// Default: '<icon> <temperature_celsius>°C'
    #[structopt(short, long)]
    pub format: Option<String>,
    /// The interval in seconds how often the weather status is updated
    ///
    /// Default: 300
    #[structopt(short, long)]
    pub interval: Option<u64>,

    /// The provider to use for pulling weather updates
    ///
    /// Available providers:
    /// OpenWeatherMap
    ///
    /// Default: OpenWeatherMap
    #[structopt(short, long = "weather-provider")]
    pub weather_provider: Option<WeatherProvider>,
    /// The API key for the corresponding weather provider
    #[structopt(short = "k", long = "weather-api-key")]
    pub weather_api_key: Option<String>,

    /// The provider to use for geolocation
    ///
    /// Available providers:
    /// Ip,
    /// Manual
    ///
    /// Default: Ip
    #[structopt(short, long = "location-provider")]
    pub location_provider: Option<LocationProvider>,
    /// Prints the current city
    #[structopt(short = "C", long = "current-city")]
    pub current_city: bool,
    /// Latitude of the location to display the weather status for
    #[structopt(long)]
    pub lat: Option<f32>,
    /// Longitude of the location to display the weather status for
    #[structopt(long)]
    pub lon: Option<f32>,
}

impl CliArgs {
    pub fn apply(&self) -> Result<()> {
        if self.debug {
            logger::init()?;
            debug!("Read {:?}", self);
        }

        if self.default_config_path {
            let path = config::file::default_config_path()
                .ok_or("Couldn't get default config path")?
                .to_str().map(|string| string.to_string())
                .ok_or("Couldn't parse default config path")?;
            println!("{}", path);
            process::exit(0);
        }

        if self.current_city {
            match IpApi::new().current_location() {
                Ok(location) => {
                    println!("{}", location.city);
                    process::exit(0)
                }
                Err(err) => {
                    println!("Couldn't get current location");
                    error!("{}", err.to_string());
                    process::exit(1)
                }
            }
        }
        Ok(())
    }
}
