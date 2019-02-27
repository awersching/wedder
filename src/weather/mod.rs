use std::collections::HashMap;

use crate::weather::weather_condition::WeatherCondition;

pub mod providers;
pub mod weather_condition;

pub trait Weather {
    fn weather_condition(&self) -> WeatherCondition;
    fn kelvin(&self) -> f32;
}

pub struct Formatter<'a> {
    format: &'a str,
    weather: Box<dyn Weather>,
    icons: &'a HashMap<String, String>,
}

impl<'a> Formatter<'a> {
    pub fn new(format: &'a str, weather: Box<dyn Weather>, icons: &'a HashMap<String, String>) -> Self {
        Self {
            format,
            weather,
            icons,
        }
    }

    pub fn format(&self) -> String {
        let condition = self.weather.weather_condition().to_string();
        let icon = self.icons.get(&condition)
            .unwrap_or(&condition);

        self.format.to_string().replace(
            "<icon>",
            icon,
        ).replace(
            "<temperature_celsius>",
            &self.temp_to_celsius().to_string(),
        ).replace(
            "<temperature_fahrenheit>",
            &self.temp_to_fahrenheit().to_string(),
        ).replace(
            "<temperature_kelvin>",
            &(self.weather.kelvin().round() as i32).to_string(),
        )
    }

    fn temp_to_celsius(&self) -> i32 {
        (self.weather.kelvin() - 273.15).round() as i32
    }

    fn temp_to_fahrenheit(&self) -> i32 {
        (1.8 * (self.weather.kelvin() - 273.15) + 32.0).round() as i32
    }
}
