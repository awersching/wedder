use crate::location::Location;
use crate::Result;
use crate::weather::providers::CurrentWeather;
use crate::weather::providers::owm::response::Response;
use crate::weather::Weather;

const URL: &str = "https://samples.openweathermap.org/data/2.5/\
        weather?q=London,uk&appid=b6907d289e10d714a6e88b30761fae22";

pub struct OwmMock;

impl CurrentWeather for OwmMock {
    #[allow(unused_variables)]
    fn weather(&self, location: &Location, api_key: &str) -> Result<Box<dyn Weather>> {
        let http = reqwest::blocking::get(URL)?;
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
    use crate::weather::providers::CurrentWeather;
    use crate::weather::providers::owm::tests::OwmMock;
    use crate::weather::weather_condition::WeatherCondition::Rain;

    #[test]
    fn weather() {
        let location = Location { city: "".to_string(), lat: 0.0, lon: 0.0 };
        let wtr = OwmMock::new()
            .weather(&location, "");
        assert!(wtr.is_ok());
        let weather = wtr.unwrap();

        assert_eq!(Rain, weather.weather_condition());

        assert_eq!(280.32, weather.kelvin());
        assert_eq!(281.15, weather.kelvin_max());
        assert_eq!(279.15, weather.kelvin_min());

        assert_eq!(1012.0, weather.pressure());
        assert_eq!(81.0, weather.humidity());
        assert_eq!(4.1, weather.wind_speed());
        assert_eq!(90.0, weather.cloud_percentage());
    }
}
