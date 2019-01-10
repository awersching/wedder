use std::error::Error;

use crate::location::Location;
use crate::util;
use crate::weather::CurrentWeather;
use crate::weather::providers::owm::response::Response;
use crate::weather::weather::Weather;
use crate::weather::WeatherCondition;

mod response;

pub struct OpenWeatherMap {
    api_key: String
}

impl CurrentWeather for OpenWeatherMap {
    fn new(api_key: &str) -> Self {
        OpenWeatherMap {
            api_key: api_key.to_string()
        }
    }

    fn current_weather(&self, location: &Location) -> Result<Weather, Box<dyn Error>> {
        let url = self.build_url(location);
        let body = util::get_retry(&url, "No connection")
            .text().unwrap();
        let response: Response = serde_json::from_str(&body)?;

        Ok(Weather::new(
            self.parse_weather_condition(&response)?,
            response.main.temp,
        ))
    }
}

impl OpenWeatherMap {
    fn build_url(&self, location: &Location) -> String {
        let base_url = "http://api.openweathermap.org/data/2.5";

        format!(
            "{}/weather?lat={}&lon={}&APPID={}",
            base_url,
            location.lat,
            location.lon,
            self.api_key
        )
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
