use std::collections::HashMap;

use crate::config::{Config, Temperature, WindSpeed};
use crate::location::Location;
use crate::weather::Weather;

macro_rules! tag {
    ($tags:expr, $value:expr) => {
        let tag = format!("<{}>", stringify!($value));
        $tags.insert(tag, $value.to_string());
    };
}

pub struct Formatter<'a> {
    config: &'a Config,
    location: Location,
    weather: Box<dyn Weather>,
}

impl<'a> Formatter<'a> {
    pub fn new(config: &'a Config, location: Location, weather: Box<dyn Weather>) -> Self {
        Self {
            config,
            location,
            weather,
        }
    }

    pub fn format(&self) -> String {
        let mut formatted = self.config.format.0.to_string();
        for (tag, value) in self.tags() {
            formatted = formatted.replace(&tag, &value);
        }
        formatted
    }

    fn tags(&self) -> HashMap<String, String> {
        let mut tags = HashMap::new();

        let city = &self.location.city;
        let icon = self.icon();
        let temperature = self.convert(self.weather.kelvin());
        let temperature_feels_like = self.convert(self.weather.kelvin_feels_like());
        let temperature_max = self.convert(self.weather.kelvin_max());
        let temperature_min = self.convert(self.weather.kelvin_min());
        let pressure = self.weather.pressure();
        let humidity = self.weather.humidity();
        let wind_speed = self.wind_speed();
        let cloud_percentage = self.weather.cloud_percentage();
        let sunrise = self.weather.sunrise().format("%H:%M");
        let sunset = self.weather.sunset().format("%H:%M");

        tag!(tags, city);
        tag!(tags, icon);
        tag!(tags, temperature);
        tag!(tags, temperature_feels_like);
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
        self.config
            .icons
            .get(&condition)
            .unwrap_or(&condition)
            .to_string()
    }

    fn convert(&self, kelvin: f32) -> i32 {
        match &self.config.units.temperature {
            Temperature::Celsius => kelvin - 273.15,
            Temperature::Fahrenheit => 1.8 * (kelvin - 273.15) + 32.0,
            Temperature::Kelvin => kelvin,
        }
        .round() as i32
    }

    fn wind_speed(&self) -> f32 {
        let wind_speed = self.weather.wind_speed();
        (match self.config.units.wind_speed {
            WindSpeed::Ms => wind_speed,
            WindSpeed::Kmh => wind_speed * 3.6,
            WindSpeed::Mph => wind_speed * (3600.0 / 1609.34),
        })
        .round()
    }
}
