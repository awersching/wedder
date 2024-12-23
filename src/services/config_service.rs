use std::{env, process};

use log::debug;
use structopt::StructOpt;

use crate::adapters::config::cli_args::CliArgs;
use crate::adapters::config::file;
use crate::model::config::Config;
use std::path::Path;

const WEDDER_WEATHER_API_KEY: &str = "WEDDER_WEATHER_API_KEY";

macro_rules! merge {
    ($config_value:expr, $args_value:expr) => {
        if let Some(x) = $args_value {
            $config_value = x
        }
    };
}

pub fn config() -> Config {
    let args = CliArgs::from_args();
    args.apply();

    let mut config = match &args.config_file {
        Some(path) => file::from_path(Path::new(path)),
        None => file::from_default_path(),
    };
    debug!("Read {:#?}", config);
    merge_config_with_args(&mut config, args);
    debug!("Merged config with args into {:#?}", config);

    if config.weather.api_key.is_empty() {
        if let Ok(key) = env::var(WEDDER_WEATHER_API_KEY) {
            config.weather.api_key = key;
        } else {
            println!("No API key");
            process::exit(1)
        }
    }
    config
}

fn merge_config_with_args(config: &mut Config, args: CliArgs) {
    merge!(config.format, args.format);
    merge!(config.interval, args.interval);
    merge!(config.units.temperature, args.temperature_unit);
    merge!(config.units.wind_speed, args.wind_speed_unit);
    merge!(config.units.distance, args.distance_unit);
    merge!(config.units.precipitation, args.precipitation_unit);
    merge!(config.weather.provider, args.weather_provider);
    merge!(config.weather.api_key, args.weather_api_key);
    merge!(config.location.provider, args.location_provider);
    merge!(config.location.location.lat, args.lat);
    merge!(config.location.location.lon, args.lon);
}

#[cfg(test)]
mod tests {
    use crate::{
        adapters::config::cli_args::CliArgs,
        model::{
            config::{
                Config, DistanceUnit, Format, Interval, PrecipitationUnit, TemperatureUnit,
                WindSpeedUnit,
            },
            location::LocationProvider,
            weather::WeatherProvider,
        },
        services::config_service::merge_config_with_args,
    };

    #[test]
    fn default() {
        let cfg_str = std::fs::read_to_string("examples/wedder.toml").unwrap();
        let file: Config = toml::from_str(&cfg_str).unwrap();
        let default = Config::default();

        assert_eq!(file.format, default.format);
        assert_eq!(file.interval, default.interval);
        assert_eq!(file.units, default.units);
        assert_eq!(file.weather, default.weather);
        assert_eq!(file.location, default.location);
        assert_eq!(file.icons, default.icons);
    }

    #[test]
    fn merge_config_with_cli_args() {
        let args = CliArgs {
            debug: false,
            default_config_path: false,
            config_file: None,
            format: Some(Format("format".to_string())),
            interval: Some(Interval(123)),
            temperature_unit: Some(TemperatureUnit::Kelvin),
            wind_speed_unit: Some(WindSpeedUnit::Ms),
            distance_unit: Some(DistanceUnit::Mile),
            precipitation_unit: Some(PrecipitationUnit::Inch),
            weather_provider: Some(WeatherProvider::OpenWeatherMap),
            weather_api_key: Some("key".to_string()),
            location_provider: Some(LocationProvider::Manual),
            lat: Some(1.0),
            lon: Some(1.0),
        };
        let mut config = Config::default();
        merge_config_with_args(&mut config, args.clone());

        assert_eq!(config.format, args.format.unwrap());
        assert_eq!(config.interval, args.interval.unwrap());
        assert_eq!(config.units.temperature, TemperatureUnit::Kelvin);
        assert_eq!(config.units.wind_speed, WindSpeedUnit::Ms);
        assert_eq!(config.units.distance, DistanceUnit::Mile);
        assert_eq!(config.units.precipitation, PrecipitationUnit::Inch);
        assert_eq!(config.weather.provider, args.weather_provider.unwrap());
        assert_eq!(config.weather.api_key, args.weather_api_key.unwrap());
        assert_eq!(config.location.provider, args.location_provider.unwrap());
        assert_eq!(config.location.location.lat, args.lat.unwrap());
        assert_eq!(config.location.location.lon, args.lon.unwrap());
    }
}
