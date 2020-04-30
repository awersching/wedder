use std::collections::HashMap;

use chrono::DateTime;
use chrono::Local;

use crate::config::Config;
use crate::config::units::{Temperature, WindSpeed};
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
    config: &'a Config,
    weather: Box<dyn Weather>,
}

impl<'a> Formatter<'a> {
    pub fn new(config: &'a Config, weather: Box<dyn Weather>) -> Self {
        Self {
            config,
            weather,
        }
    }

    pub fn format(&self) -> String {
        let mut formatted = self.config.format.to_string();
        for (tag, value) in self.tags() {
            formatted = formatted.replace(&tag, &value);
        }
        formatted
    }

    fn tags(&self) -> HashMap<String, String> {
        let mut tags = HashMap::new();

        let icon = self.icon();
        let (temperature, temperature_max, temperature_min) = self.temperature();
        let pressure = self.weather.pressure();
        let humidity = self.weather.humidity();
        let wind_speed = self.wind_speed();
        let cloud_percentage = self.weather.cloud_percentage();
        let sunrise = self.weather.sunrise()
            .format("%H:%M");
        let sunset = self.weather.sunset()
            .format("%H:%M");

        tag!(tags, icon);
        tag!(tags, temperature);
        tag!(tags, temperature_max);
        tag!(tags, temperature_min);
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
        self.config.icons.get(&condition)
            .unwrap_or(&condition).to_string()
    }

    fn temperature(&self) -> (i32, i32, i32) {
        let kelvin = self.weather.kelvin();
        let kelvin_max = self.weather.kelvin_max();
        let kelvin_min = self.weather.kelvin_min();

        match &self.config.units.temperature {
            Temperature::Celsius =>
                (celsius(kelvin), celsius(kelvin_max), celsius(kelvin_min)),
            Temperature::Fahrenheit =>
                (fahrenheit(kelvin), fahrenheit(kelvin_max), fahrenheit(kelvin_min)),
            Temperature::Kelvin =>
                (kelvin.round() as i32, kelvin_max.round() as i32, kelvin_min.round() as i32),
        }
    }

    fn wind_speed(&self) -> f32 {
        let wind_speed = self.weather.wind_speed();
        (match self.config.units.wind_speed {
            WindSpeed::Ms => wind_speed,
            WindSpeed::Kmh => wind_speed * 3.6,
            WindSpeed::Mph => wind_speed * (3600.0 / 1609.34),
        }).round()
    }
}

fn celsius(kelvin: f32) -> i32 {
    (kelvin - 273.15).round() as i32
}

fn fahrenheit(kelvin: f32) -> i32 {
    (1.8 * (kelvin - 273.15) + 32.0).round() as i32
}
