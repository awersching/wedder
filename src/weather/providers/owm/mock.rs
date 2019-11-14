use crate::location::Location;
use crate::Result;
use crate::weather::providers::CurrentWeather;
use crate::weather::providers::owm::response::Response;
use crate::weather::Weather;

pub struct OwmMock;

impl CurrentWeather for OwmMock {
    #[allow(unused_variables)]
    fn current_weather(&self, location: &Location, api_key: &str) -> Result<Box<dyn Weather>> {
        let url = "https://samples.openweathermap.org/data/2.5/\
        weather?q=London,uk&appid=b6907d289e10d714a6e88b30761fae22";
        let mut http = reqwest::get(url)?;

        let response: Response = serde_json::from_str(&http.text()?)?;
        Ok(Box::new(response))
    }
}

impl OwmMock {
    pub fn new() -> Self {
        Self
    }
}
