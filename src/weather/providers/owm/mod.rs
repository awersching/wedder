use log::debug;

use crate::location::Location;
use crate::util;
use crate::weather::providers::CurrentWeather;
use crate::weather::providers::owm::response::Response;
use crate::weather::Weather;
use crate::weather::weather_condition::WeatherCondition;

mod response;

pub struct OpenWeatherMap {}

impl CurrentWeather for OpenWeatherMap {
    fn current_weather(&self, location: &Location, api_key: &str) -> util::Result<Weather> {
        let url = self.build_url(location, api_key);
        let body = util::get_retry(&url)
            .text().unwrap();
        let response: Response = serde_json::from_str(&body)?;
        debug!("Parsed response {:?}", response);

        Ok(Weather::new(
            self.parse_weather_condition(&response)?,
            response.main.temp,
        ))
    }
}

impl OpenWeatherMap {
    pub fn new() -> Self {
        OpenWeatherMap {}
    }

    fn build_url(&self, location: &Location, api_key: &str) -> String {
        let base_url = "http://api.openweathermap.org/data/2.5";

        let url = format!(
            "{}/weather?lat={}&lon={}&APPID={}",
            base_url,
            location.lat,
            location.lon,
            api_key
        );
        debug!("Built URL {}", url);
        url
    }

    fn parse_weather_condition(&self, response: &Response) -> Result<WeatherCondition, String> {
        let id = response.weather[0].id;
        let first_digit = id.to_string()[0..1].parse::<i32>().unwrap();

        match first_digit {
            2 => Ok(WeatherCondition::Thunderstorm),

            // rain
            3 => match id {
                300 => Ok(WeatherCondition::Rain),
                301 => Ok(WeatherCondition::Rain),
                302 => Ok(WeatherCondition::HeavyRain),
                310 => Ok(WeatherCondition::Rain),
                311 => Ok(WeatherCondition::HeavyRain),
                312 => Ok(WeatherCondition::HeavyRain),
                313 => Ok(WeatherCondition::HeavyRain),
                314 => Ok(WeatherCondition::HeavyRain),
                321 => Ok(WeatherCondition::HeavyRain),
                _ => Err("Undefined weather condition".to_string())
            },
            5 => match id {
                500 => Ok(WeatherCondition::Rain),
                501 => Ok(WeatherCondition::HeavyRain),
                502 => Ok(WeatherCondition::HeavyRain),
                503 => Ok(WeatherCondition::HeavyRain),
                504 => Ok(WeatherCondition::HeavyRain),
                511 => Ok(WeatherCondition::HeavyRain),
                520 => Ok(WeatherCondition::Rain),
                521 => Ok(WeatherCondition::HeavyRain),
                522 => Ok(WeatherCondition::HeavyRain),
                531 => Ok(WeatherCondition::HeavyRain),
                _ => Err("Undefined weather condition".to_string())
            },

            6 => Ok(WeatherCondition::Snow),
            7 => Ok(WeatherCondition::Mist),

            // clear sky and clouds
            8 => match id {
                800 => Ok(WeatherCondition::ClearSky),
                801 => Ok(WeatherCondition::FewClouds),
                802 => Ok(WeatherCondition::Clouds),
                803 => Ok(WeatherCondition::ManyClouds),
                804 => Ok(WeatherCondition::ManyClouds),
                _ => Err("Undefined weather condition".to_string())
            },

            _ => Err("Undefined weather condition".to_string())
        }
    }
}
