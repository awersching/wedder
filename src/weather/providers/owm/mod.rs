use std::process;

use log::debug;

use crate::location::Location;
use crate::util;
use crate::util::Result;
use crate::weather::providers::CurrentWeather;
use crate::weather::providers::owm::response::Response;
use crate::weather::Weather;

mod response;
pub mod mock;

const BASE_URL: &str = "http://api.openweathermap.org/data/2.5";

pub struct OpenWeatherMap;

impl CurrentWeather for OpenWeatherMap {
    fn current_weather(&self, location: &Location, api_key: &str) -> Result<Box<dyn Weather>> {
        let url = format!(
            "{}/weather?lat={}&lon={}&APPID={}",
            BASE_URL,
            location.lat,
            location.lon,
            api_key
        );

        debug!("Querying {} ...", url);
        let mut http_response = util::get_retry(&url);
        debug!("HTTP {}", http_response.status().to_string());
        if http_response.status().as_u16() == 401 {
            println!("Invalid/unauthorized API key");
            process::exit(1)
        }

        let response: Response = serde_json::from_str(&http_response.text()?)?;
        debug!("{:?}", response);
        Ok(Box::new(response))
    }
}

impl OpenWeatherMap {
    pub fn new() -> Self {
        OpenWeatherMap
    }
}
