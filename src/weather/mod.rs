use std::collections::HashMap;

use chrono::DateTime;
use chrono::Local;

use crate::weather::weather_condition::WeatherCondition;

pub mod providers;
pub mod weather_condition;

pub trait Weather {
    fn weather_condition(&self) -> WeatherCondition;

    fn kelvin(&self) -> f32;
    fn kelvin_max(&self) -> f32;
    fn kelvin_min(&self) -> f32;

    fn pressure(&self) -> f32;
    fn humidity(&self) -> f32;
    fn wind_speed(&self) -> f32;
    fn cloud_percentage(&self) -> f32;

    fn sunrise(&self) -> DateTime<Local>;
    fn sunset(&self) -> DateTime<Local>;
}

macro_rules! tag {
    ($tags:expr, $value:expr) => {
        let tag = format!("<{}>", stringify!($value));
        $tags.insert(tag, $value.to_string());
    };
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
        let mut formatted = self.format.to_string();

        for (tag, value) in self.tags() {
            formatted = formatted.replace(&tag, &value);
        }
        formatted
    }

    fn tags(&self) -> HashMap<String, String> {
        let mut tags = HashMap::new();

        let icon = self.icon();
        let temperature_kelvin = self.weather.kelvin();
        let kelvin_max = self.weather.kelvin_max();
        let kelvin_min = self.weather.kelvin_min();
        let temperature_celsius = self.to_celsius(temperature_kelvin);
        let celsius_max = self.to_celsius(kelvin_max);
        let celsius_min = self.to_celsius(kelvin_min);
        let temperature_fahrenheit = self.to_fahrenheit(temperature_kelvin);
        let fahrenheit_max = self.to_fahrenheit(kelvin_max);
        let fahrenheit_min = self.to_fahrenheit(kelvin_min);
        let pressure = self.weather.pressure();
        let humidity = self.weather.humidity();
        let wind_speed = self.weather.wind_speed();
        let cloud_percentage = self.weather.cloud_percentage();
        let sunrise = self.weather.sunrise()
            .format("%H:%M");
        let sunset = self.weather.sunset()
            .format("%H:%M");

        tag!(tags, icon);
        tag!(tags, temperature_kelvin);
        tag!(tags, kelvin_max);
        tag!(tags, kelvin_min);
        tag!(tags, temperature_celsius);
        tag!(tags, celsius_max);
        tag!(tags, celsius_min);
        tag!(tags, temperature_fahrenheit);
        tag!(tags, fahrenheit_max);
        tag!(tags, fahrenheit_min);
        tag!(tags, pressure);
        tag!(tags, humidity);
        tag!(tags, wind_speed);
        tag!(tags, cloud_percentage);
        tag!(tags, sunrise);
        tag!(tags, sunset);

        tags
    }

    fn icon(&self) -> String {
        let condition = self.weather.weather_condition().to_string();
        self.icons.get(&condition)
            .unwrap_or(&condition).to_string()
    }

    fn to_celsius(&self, kelvin: f32) -> i32 {
        (kelvin - 273.15).round() as i32
    }

    fn to_fahrenheit(&self, kelvin: f32) -> i32 {
        (1.8 * (kelvin - 273.15) + 32.0).round() as i32
    }
}
