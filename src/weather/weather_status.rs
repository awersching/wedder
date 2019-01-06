pub struct WeatherStatus {}

impl WeatherStatus {
    pub fn new() -> Self {
        WeatherStatus {}
    }
}

impl ToString for WeatherStatus {
    fn to_string(&self) -> String {
        String::from("Weather status")
    }
}
