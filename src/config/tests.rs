use std::fs;

use crate::config::{Config, file};
use crate::config::cli_args::CliArgs;
use crate::config::units::Temperature::Kelvin;
use crate::config::units::WindSpeed::Ms;
use crate::location::LocationProvider::Manual;
use crate::weather::providers::WeatherProvider::OwmMock;

#[test]
fn default() {
    let cfg_str = fs::read_to_string("examples/wedder.toml").unwrap();
    let file: Config = toml::from_str(&cfg_str).unwrap();
    let default = Config::default();

    assert_eq!(file.format, default.format);
    assert_eq!(file.interval, Some(300));
    assert_eq!(None, default.interval);
    assert_eq!(file.units, default.units);
    assert_eq!(file.weather, default.weather);
    assert_eq!(file.location, default.location);
    assert_eq!(file.icons, default.icons);
}

#[test]
fn merge() {
    let args = CliArgs {
        debug: false,
        current_city: false,
        default_config_path: false,
        config_file: Some("examples/wedder.toml".to_string()),
        format: Some("format".to_string()),
        interval: None,
        temperature_unit: Some(Kelvin),
        wind_speed_unit: Some(Ms),
        weather_provider: Some(OwmMock),
        weather_api_key: Some("key".to_string()),
        location_provider: Some(Manual),
        lat: Some(0.0),
        lon: Some(0.0),
    };
    let config = Config::new(args.clone());

    assert_eq!(config.format, args.format.unwrap());
    assert_eq!(args.interval, None);
    assert_eq!(config.interval, Some(300));
    assert_eq!(config.units.temperature, Kelvin);
    assert_eq!(config.units.wind_speed, Ms);
    assert_eq!(config.weather.provider, args.weather_provider.unwrap());
    assert_eq!(config.weather.api_key, args.weather_api_key.unwrap());
    assert_eq!(config.location.provider, args.location_provider.unwrap());
    assert_eq!(config.location.location.lat, args.lat.unwrap());
    assert_eq!(config.location.location.lon, args.lon.unwrap());
}

#[test]
fn path_not_found() {
    let config = file::load_config(&[""].iter().collect());
    assert!(config.is_some());
    assert_eq!(config.unwrap(), Config::default());
}
