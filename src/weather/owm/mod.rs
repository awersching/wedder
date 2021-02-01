use std::process;

use log::debug;

use crate::http::get_retry;
use crate::location::Location;
use crate::weather::owm::response::Response;
use crate::weather::CurrentWeather;
use crate::weather::Weather;
use crate::Result;

mod response;

const URL: &str = "http://api.openweathermap.org/data/2.5";
const MOCK_URL: &str = "https://samples.openweathermap.org/data/2.5/\
        weather?q=London,uk&appid=b6907d289e10d714a6e88b30761fae22";

pub struct OpenWeatherMap;

impl CurrentWeather for OpenWeatherMap {
    fn weather(&self, location: &Location, api_key: &str) -> Result<Box<dyn Weather>> {
        let url = format!(
            "{}/weather?lat={}&lon={}&APPID={}",
            URL, location.lat, location.lon, api_key
        );

        debug!("Querying {} ...", url);
        let http_response = get_retry(&url);
        debug!("HTTP {}", http_response.status().to_string());
        if http_response.status().as_u16() == 401 {
            println!("Invalid/unauthorized API key");
            process::exit(1)
        }

        let response: Response = serde_json::from_str(&http_response.text()?)?;
        debug!("{:#?}", response);
        Ok(Box::new(response))
    }
}

impl OpenWeatherMap {
    pub fn new() -> Self {
        Self
    }
}

pub struct OwmMock;

impl CurrentWeather for OwmMock {
    #[allow(unused_variables)]
    fn weather(&self, location: &Location, api_key: &str) -> Result<Box<dyn Weather>> {
        let http = reqwest::blocking::get(MOCK_URL)?;
        let response: Response = serde_json::from_str(&http.text()?)?;
        Ok(Box::new(response))
    }
}

impl OwmMock {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use crate::location::Location;
    use crate::weather::owm::OwmMock;
    use crate::weather::weather_condition::WeatherCondition::Rain;
    use crate::weather::CurrentWeather;

    #[test]
    fn weather() {
        let location = Location {
            city: "".to_string(),
            lat: 0.0,
            lon: 0.0,
        };
        let weather = OwmMock::new().weather(&location, "");
        assert!(weather.is_ok());
        let weather = weather.unwrap();

        assert_eq!(Rain, weather.weather_condition());

        assert_eq!(280.32, weather.temp().0);
        // not available in mock
        assert!(weather.temp_feels_like().is_none());
        assert_eq!(281.15, weather.temp_max().unwrap().0);
        assert_eq!(279.15, weather.temp_min().unwrap().0);

        assert_eq!(1012.0, weather.pressure().unwrap().0);
        assert_eq!(81.0, weather.humidity().unwrap().0);
        assert_eq!(4.1, weather.wind_speed().unwrap().0);
        assert_eq!(90.0, weather.clouds().unwrap().0);
        assert_eq!(10000.0, weather.visibility().unwrap().0);
        // not available in mock
        assert!(weather.precipitation().is_none());
    }
}
