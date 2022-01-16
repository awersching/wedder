use std::process;

use log::debug;
use structopt::StructOpt;

use crate::config;
use crate::config::{
    DistanceUnit, Format, Interval, PrecipitationUnit, TemperatureUnit, WindSpeedUnit,
};
use crate::location::LocationProvider;
use crate::logger;
use crate::weather::WeatherProvider;

#[derive(Debug, StructOpt, Clone)]
#[structopt(author, about, setting = structopt::clap::AppSettings::AllowLeadingHyphen)]
pub struct CliArgs {
    /// Enables verbose debug output
    #[structopt(short = "d", long)]
    pub debug: bool,
    /// Prints the default config path
    #[structopt(short = "p", long)]
    pub default_config_path: bool,
    /// Path to an alternative config file
    #[structopt(short = "c", long)]
    pub config_file: Option<String>,

    /// The format to display the weather status in
    ///
    /// Available tags:
    /// <city>
    /// <icon>
    /// <temperature>
    /// <temperature_feels_like>
    /// <temperature_max>
    /// <temperature_min>
    /// <dew_point>
    /// <precipitation>
    /// <precipitation_chance>
    /// <clouds>
    /// <humidity>
    /// <visibility>
    /// <wind_speed>
    /// <pressure>
    /// <uv_index>
    /// <air_quality_index>
    /// <sunrise>
    /// <sunset>
    ///
    /// Default: '<icon> <temperature>°C'
    #[structopt(short = "f", long)]
    pub format: Option<Format>,
    /// The interval in seconds how often the weather status is updated
    ///
    /// If a negative interval is specified, wedder exits after printing the weather once
    #[structopt(short = "i", long)]
    pub interval: Option<Interval>,

    /// The unit of temperature values
    ///
    /// Available units:
    /// Celsius,
    /// Fahrenheit,
    /// Kelvin
    ///
    /// Default: Celsius
    #[structopt(short = "t", long)]
    pub temperature_unit: Option<TemperatureUnit>,
    /// The unit for the wind speed
    ///
    /// Available units:
    /// Ms,
    /// Kmh,
    /// Mph
    ///
    /// Default: Kmh
    #[structopt(short = "s", long)]
    pub wind_speed_unit: Option<WindSpeedUnit>,
    /// The unit of distances
    ///
    /// Available units:
    /// Meter,
    /// Kilometer,
    /// Mile
    ///
    /// Default: Kilometer
    #[structopt(short = "D", long)]
    pub distance_unit: Option<DistanceUnit>,
    /// The unit of the precipitation
    ///
    /// Available units:
    /// Millimeter,
    /// Inch
    ///
    /// Default: Millimeter
    #[structopt(short = "P", long)]
    pub precipitation_unit: Option<PrecipitationUnit>,

    /// The provider to use for pulling weather updates
    ///
    /// Available providers:
    /// OpenWeatherMap
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
    /// IpApi,
    /// Manual
    ///
    /// Default: IpApi
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
            self.debug();
        }
        if self.default_config_path {
            if let Err(err) = Self::default_config_path() {
                println!("{}", err);
                process::exit(1);
            }
        }
    }

    fn debug(&self) {
        if let Err(err) = logger::init() {
            println!("Error initializing logger");
            println!("{}", err);
            process::exit(1);
        }
        debug!("Read {:#?}", self);
    }

    fn default_config_path() -> crate::Result<()> {
        let path = config::file::default_config_path()
            .ok_or("Couldn't get default config path")?
            .to_str()
            .map(std::string::ToString::to_string)
            .ok_or("Couldn't parse default config path")?;
        println!("{}", path);
        process::exit(0);
    }
}
