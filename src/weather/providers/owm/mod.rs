use std::error::Error;
use std::thread;
use std::time::Duration;

use crate::weather::CurrentWeather;
use crate::weather::providers::owm::response::Response;
use crate::weather::weather::Weather;
use crate::weather::WeatherCondition;

mod response;

pub struct OpenWeatherMap {
    api_key: String
}

impl CurrentWeather for OpenWeatherMap {
    fn new(api_key: String) -> Self {
        OpenWeatherMap {
            api_key
        }
    }

    fn current_weather(&self, location: &String) -> Result<Weather, Box<dyn Error>> {
        let body = self.request_current_weather(location)
            .text().unwrap();
        let response: Response = serde_json::from_str(&body)?;

        Ok(Weather::new(
            self.parse_weather_condition(&response)?,
            response.main.temp,
        ))
    }
}

impl OpenWeatherMap {
    fn request_current_weather(&self, location: &String) -> reqwest::Response {
        let base_url = "http://api.openweathermap.org/data/2.5";

        let url = format!(
            "{}/weather?q={}&APPID={}",
            base_url,
            location,
            self.api_key
        );

        // if error wait for reconnection
        let mut result = reqwest::get(&url);
        while result.is_err() {
            println!("No connection");
            thread::sleep(Duration::from_secs(5));
            result = reqwest::get(&url);
        }
        result.unwrap()
    }

    fn parse_weather_condition(&self, response: &Response) -> Result<WeatherCondition, String> {
        // owm has different icons for day and night (third char), we do not
        let icon_code = &response.weather[0].icon[0..2];

        match icon_code {
            //day
            "01" => Ok(WeatherCondition::ClearSky),
            "02" => Ok(WeatherCondition::FewClouds),
            "03" => Ok(WeatherCondition::Clouds),
            "04" => Ok(WeatherCondition::ManyClouds),
            "10" => Ok(WeatherCondition::Rain),
            "09" => Ok(WeatherCondition::HeavyRain),
            "11" => Ok(WeatherCondition::Thunderstorm),
            "13" => Ok(WeatherCondition::Snow),
            "50" => Ok(WeatherCondition::Mist),

            _ => Err("Undefined weather condition".to_string())
        }
    }
}
