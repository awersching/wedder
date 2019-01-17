use std::collections::HashMap;

use crate::weather::weather_condition::WeatherCondition;

pub mod providers;
pub mod weather_condition;

#[derive(Debug)]
pub struct Weather {
    weather_condition: WeatherCondition,
    kelvin: f32,
}

impl Weather {
    pub fn new(weather_condition: WeatherCondition, kelvin: f32) -> Self {
        Weather {
            weather_condition,
            kelvin,
        }
    }

    pub fn format(&self, format: &str, icons: &HashMap<String, String>) -> String {
        let icon = icons.get(&self.weather_condition.to_string())
            .unwrap();

        format.to_string().replace(
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
            &(self.kelvin.round() as i32).to_string(),
        )
    }

    fn temp_to_celsius(&self) -> i32 {
        (self.kelvin - 273.15).round() as i32
    }

    fn temp_to_fahrenheit(&self) -> i32 {
        (1.8 * (self.kelvin - 273.15) + 32.0).round() as i32
    }
}
