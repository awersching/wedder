use log::debug;

use crate::location::Location;
use crate::util;
use crate::weather::error::UndefinedCondition;
use crate::weather::providers::CurrentWeather;
use crate::weather::providers::owm::response::OwmResponse;
use crate::weather::providers::owm::response::OwmWeather;
use crate::weather::Weather;
use crate::weather::weather_condition::WeatherCondition;

mod response;

pub struct OpenWeatherMap;

const BASE_URL: &str = "http://api.openweathermap.org/data/2.5";

impl CurrentWeather for OpenWeatherMap {
    fn current_weather(&self, location: &Location, api_key: &str) -> util::Result<Weather> {
        let url = self.build_url(location, api_key);
        let body = util::get_retry(&url).text()?;
        let response: OwmResponse = serde_json::from_str(&body)?;
        debug!("Parsed response {:?}", response);

        let weather_condition = if response.weather.len() > 1 {
            self.combined_weather_condition(&response.weather)?
        } else {
            self.weather_condition(&response.weather[0])?
        };
        Ok(Weather::new(
            weather_condition,
            response.main.temp,
        ))
    }
}

impl OpenWeatherMap {
    pub fn new() -> Self {
        OpenWeatherMap
    }

    fn build_url(&self, location: &Location, api_key: &str) -> String {
        let url = format!(
            "{}/weather?lat={}&lon={}&APPID={}",
            BASE_URL,
            location.lat,
            location.lon,
            api_key
        );
        debug!("Built URL {}", url);
        url
    }

    fn weather_condition(&self, owm_weather: &OwmWeather) -> util::Result<WeatherCondition> {
        let id = owm_weather.id;
        let first_digit = id.to_string()[0..1].parse::<i32>()?;

        match first_digit {
            2 => Ok(WeatherCondition::Thunderstorm),

            // rain
            3 => match id {
                300 | 301 | 310 => Ok(WeatherCondition::Rain),
                302 | 311 | 312 | 313 | 314 | 321 => Ok(WeatherCondition::HeavyRain),
                _ => Err(UndefinedCondition.into())
            },
            5 => match id {
                500 | 520 => Ok(WeatherCondition::Rain),
                501 | 502 | 503 | 504 | 511 | 521 | 522 | 531 => Ok(WeatherCondition::HeavyRain),
                _ => Err(UndefinedCondition.into())
            },

            6 => Ok(WeatherCondition::Snow),
            7 => Ok(WeatherCondition::Mist),

            // clear sky and clouds
            8 => match id {
                800 => Ok(WeatherCondition::ClearSky),
                801 => Ok(WeatherCondition::FewClouds),
                802 => Ok(WeatherCondition::Clouds),
                803 | 804 => Ok(WeatherCondition::ManyClouds),
                _ => Err(UndefinedCondition.into())
            },

            _ => Err(UndefinedCondition.into())
        }
    }

    fn combined_weather_condition(&self, owm_weather: &[OwmWeather]) -> util::Result<WeatherCondition> {
        let condition1 = self.weather_condition(&owm_weather[0])?;
        let condition2 = self.weather_condition(&owm_weather[1])?;

        let sun = condition1 == WeatherCondition::ClearSky ||
            condition2 == WeatherCondition::ClearSky;
        let rain = condition1 == WeatherCondition::Rain ||
            condition2 == WeatherCondition::Rain ||
            condition1 == WeatherCondition::HeavyRain ||
            condition2 == WeatherCondition::HeavyRain;

        Ok(if rain && sun {
            WeatherCondition::RainSun
        } else {
            condition1
        })
    }
}
