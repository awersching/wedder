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

    fn current_weather(&self, location: &String) -> Weather {
        let body = self.request_current_weather(location);
        let response: Response = serde_json::from_str(&body).unwrap();

        Weather::new(
            self.parse_weather_condition(&response),
            response.main.temp,
        )
    }
}

impl OpenWeatherMap {
    fn request_current_weather(&self, location: &String) -> String {
        let base_url = "http://api.openweathermap.org/data/2.5";

        let url = format!(
            "{}/weather?q={}&APPID={}",
            base_url,
            location,
            self.api_key
        );
        reqwest::get(&url).unwrap().text().unwrap()
    }

    fn parse_weather_condition(&self, response: &Response) -> WeatherCondition {
        // owm has different icons for day and night (third char), we do not
        let icon_code = &response.weather[0].icon[0..2];

        match icon_code {
            //day
            "01" => WeatherCondition::ClearSky,
            "02" => WeatherCondition::FewClouds,
            "03" => WeatherCondition::Clouds,
            "04" => WeatherCondition::ManyClouds,
            "10" => WeatherCondition::Rain,
            "09" => WeatherCondition::HeavyRain,
            "11" => WeatherCondition::Thunderstorm,
            "13" => WeatherCondition::Snow,
            "50" => WeatherCondition::Mist,

            // TODO: better error handling
            _ => panic!("Undefined weather condition")
        }
    }
}
