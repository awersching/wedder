use crate::settings::Settings;
use crate::weather::Weather;
use crate::weather::weather_status::WeatherStatus;

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
        info!("{:?}", body);

        WeatherStatus::new()
    }
}

impl OpenWeatherMap {
    fn request_current_weather(&self, location: String) -> String {
        let base_url = "http://api.openweathermap.org/data/2.5";

        let url = format!(
            "{}/weather?q={}&APPID={}",
            base_url,
            location,
            self.weather_api_key
        );
        reqwest::get(&url).unwrap().text().unwrap()
    }
}
