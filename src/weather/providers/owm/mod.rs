use crate::settings::Settings;
use crate::weather::providers::owm::response::Response;
use crate::weather::Weather;
use crate::weather::weather_condition::WeatherCondition;
use crate::weather::weather_status::WeatherStatus;

mod response;

pub struct OpenWeatherMap {
    weather_api_key: String
}

impl Weather for OpenWeatherMap {
    fn new(settings: &Settings) -> Self {
        OpenWeatherMap {
            weather_api_key: settings.weather_api_key.clone()
        }
    }

    fn current_weather(&self) -> WeatherStatus {
        let body = self.request_current_weather(String::from("London"));
        let response: Response = serde_json::from_str(&body).unwrap();

        WeatherStatus::new(
            self.parse_weather_condition(&response),
            // TODO: round
            response.main.temp as i32,
            String::from("°C"),
        )
    }
}

impl OpenWeatherMap {
    fn request_current_weather(&self, location: String) -> String {
        let base_url = "http://api.openweathermap.org/data/2.5";

        let url = format!(
            "{}/weather?q={}&units=metric&APPID={}",
            base_url,
            location,
            self.weather_api_key
        );
        reqwest::get(&url).unwrap().text().unwrap()
    }

    fn parse_weather_condition(&self, response: &Response) -> WeatherCondition {
        match response.weather[0].icon.as_ref() {
            //day
            "01d" => WeatherCondition::ClearSky,
            "02d" => WeatherCondition::FewClouds,
            "03d" => WeatherCondition::ScatteredClouds,
            "04d" => WeatherCondition::BrokenClouds,
            "09d" => WeatherCondition::ShowerRain,
            "10d" => WeatherCondition::Rain,
            "11d" => WeatherCondition::Thunderstorm,
            "13d" => WeatherCondition::Snow,
            "50d" => WeatherCondition::Mist,

            //night
            "01n" => WeatherCondition::ClearSky,
            "02n" => WeatherCondition::FewClouds,
            "03n" => WeatherCondition::ScatteredClouds,
            "04n" => WeatherCondition::BrokenClouds,
            "09n" => WeatherCondition::ShowerRain,
            "10n" => WeatherCondition::Rain,
            "11n" => WeatherCondition::Thunderstorm,
            "13n" => WeatherCondition::Snow,
            "50n" => WeatherCondition::Mist,

            // TODO: better error handling
            _ => panic!("Undefined weather condition")
        }
    }
}
