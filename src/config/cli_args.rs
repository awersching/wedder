use std::process;

use log::debug;
use log::error;
use structopt::StructOpt;

use crate::config;
use crate::config::units::{Temperature, WindSpeed};
use crate::location::CurrentLocation;
use crate::location::ip_api::IpApi;
use crate::location::LocationProvider;
use crate::logger;
use crate::Result;
use crate::weather::providers::WeatherProvider;

#[derive(Debug, StructOpt, Clone)]
#[structopt(author, about, setting = structopt::clap::AppSettings::AllowLeadingHyphen)]
pub struct CliArgs {
    /// Enables verbose debug output
    #[structopt(short = "d", long)]
    pub debug: bool,
    /// Prints the current city
    #[structopt(short = "C", long)]
    pub current_city: bool,
    /// Prints the default config path
    #[structopt(short = "p", long)]
    pub default_config_path: bool,
    /// Path to an alternative config file
    #[structopt(short = "c", long)]
    pub config_file: Option<String>,

    /// The format to display the weather status in
    ///
    /// Available tags:
    /// <icon>,
    /// <temperature>,
    /// <temperature_max>,
    /// <temperature_min>,
    /// <pressure>,
    /// <humidity>,
    /// <wind_speed>,
    /// <cloud_percentage>,
    /// <sunrise>,
    /// <sunset>
    ///
    /// Default: '<icon> <temperature>°C'
    #[structopt(short = "f", long)]
    pub format: Option<String>,
    /// The interval in seconds how often the weather status is updated
    ///
    /// If no interval is specified, wedder exits after printing the weather once
    #[structopt(short = "i", long)]
    pub interval: Option<u64>,

    /// The unit of temperature values
    ///
    /// Available units:
    /// Celsius,
    /// Fahrenheit,
    /// Kelvin
    ///
    /// Default: Celsius
    #[structopt(short = "t", long)]
    pub temperature_unit: Option<Temperature>,
    /// The unit for the wind speed
    ///
    /// Available units:
    /// Ms,
    /// Kmh,
    /// Mph
    ///
    /// Default: Kmh
    #[structopt(short = "s", long)]
    pub wind_speed_unit: Option<WindSpeed>,

    /// The provider to use for pulling weather updates
    ///
    /// Available providers:
    /// OpenWeatherMap,
    /// OwmMock (for testing purposes)
    ///
    /// Default: OpenWeatherMap
    #[structopt(short = "w", long)]
    pub weather_provider: Option<WeatherProvider>,
    /// The API key for the corresponding weather provider
    #[structopt(short = "k", long)]
    pub weather_api_key: Option<String>,

    /// The provider to use for geolocation
    ///
    /// Available providers:
    /// Ip,
    /// Manual
    ///
    /// Default: Ip
    #[structopt(short = "l", long)]
    pub location_provider: Option<LocationProvider>,
    /// Latitude of the location to display the weather status for
    #[structopt(long)]
    pub lat: Option<f32>,
    /// Longitude of the location to display the weather status for
    #[structopt(long)]
    pub lon: Option<f32>,
}

impl CliArgs {
    pub fn apply(&self) {
        if self.debug {
            self.debug()
        }
        if self.current_city {
            self.current_city()
        }
        if self.default_config_path {
            if let Err(err) = self.default_config_path() {
                println!("{}", err.to_string());
                process::exit(1);
            }
        }
    }

    fn debug(&self) {
        if let Err(err) = logger::init() {
            println!("Error initializing logger");
            println!("{}", err.to_string());
            process::exit(1);
        }
        debug!("Read {:?}", self);
    }
    fn current_city(&self) {
        match IpApi::new().location() {
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

    fn default_config_path(&self) -> Result<()> {
        let path = config::file::default_config_path()
            .ok_or("Couldn't get default config path")?
            .to_str().map(std::string::ToString::to_string)
            .ok_or("Couldn't parse default config path")?;
        println!("{}", path);
        process::exit(0);
    }
}
