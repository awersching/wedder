use structopt::StructOpt;

use crate::location::LocationProvider;
use crate::weather::providers::WeatherProvider;

#[derive(Debug, StructOpt)]
#[structopt()]
pub struct CmdArgs {
    /// Enables verbose debug output
    #[structopt(short, long)]
    pub debug: bool,

    /// Prints the default config path
    #[structopt(short, long = "print-default-config-path")]
    pub print_default_config_path: bool,
    /// Path to an alternative config file
    #[structopt(short, long = "config-file")]
    pub config_file: Option<String>,

    /// The format to display the weather status in
    ///
    /// Available tags:
    /// <icon>,
    /// <temperature_celsius>,
    /// <temperature_fahrenheit>,
    /// <temperature_kelvin>
    ///
    /// E.g. '<icon> <temperature_celsius>°C'
    #[structopt(short, long)]
    pub format: Option<String>,
    /// The interval in seconds how often the weather status is updated
    #[structopt(short, long)]
    pub interval: Option<i32>,

    /// The provider to use for pulling weather updates
    ///
    /// Available providers:
    /// OpenWeatherMap
    #[structopt(long = "weather-provider")]
    pub weather_provider: Option<WeatherProvider>,
    /// The API key for the corresponding weather provider
    #[structopt(long = "weather-api-key")]
    pub weather_api_key: Option<String>,

    /// The provider to use for geolocation
    ///
    /// Available providers:
    /// Ip,
    /// Manual
    #[structopt(long = "location-provider")]
    pub location_provider: Option<LocationProvider>,
    /// Latitude of the location to display the weather status for
    #[structopt(long)]
    pub lat: Option<f32>,
    /// Longitude of the location to display the weather status for
    #[structopt(long)]
    pub lon: Option<f32>,
}
