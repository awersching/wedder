use std::collections::HashMap;

use crate::config::Config;
use crate::location::Location;
use crate::weather::{Convert, Weather};

macro_rules! tag {
    ($tags:expr, $option:expr) => {
        let tag = format!("<{}>", stringify!($option));
        if let Some(value) = $option {
            $tags.insert(tag, value.to_string());
        } else {
            $tags.insert(tag, "N/A".to_string());
        }
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

        let temperature = self.convert(self.weather.temp());
        let temperature_feels_like = self.convert(self.weather.temp_feels_like());
        let temperature_max = self.convert(self.weather.temp_max());
        let temperature_min = self.convert(self.weather.temp_min());
        let dew_point = self.convert(self.weather.dew_point());

        let precipitation = self.convert(self.weather.precipitation());
        let precipitation_chance = self.weather.precipitation_chance();
        let clouds = self.weather.clouds();
        let humidity = self.weather.humidity();
        let visibility = self.convert(self.weather.visibility());
        let wind_speed = self.convert(self.weather.wind_speed());
        let pressure = self.weather.pressure();
        let uv_index = self.weather.uvi();
        let air_quality_index = self.weather.aqi();

        let sunrise = self.weather.sunrise().map(|time| time.format("%H:%M"));
        let sunset = self.weather.sunset().map(|time| time.format("%H:%M"));

        tag!(tags, city);
        tag!(tags, icon);
        tag!(tags, temperature);
        tag!(tags, temperature_feels_like);
        tag!(tags, temperature_max);
        tag!(tags, temperature_min);
        tag!(tags, dew_point);
        tag!(tags, precipitation);
        tag!(tags, precipitation_chance);
        tag!(tags, clouds);
        tag!(tags, humidity);
        tag!(tags, visibility);
        tag!(tags, wind_speed);
        tag!(tags, pressure);
        tag!(tags, uv_index);
        tag!(tags, air_quality_index);
        tag!(tags, sunrise);
        tag!(tags, sunset);
        tags
    }

    fn icon(&self) -> Option<String> {
        let condition = self.weather.weather_condition()?.to_string();
        let icon = self
            .config
            .icons
            .get(&condition)
            .unwrap_or(&condition)
            .to_string();
        Some(icon)
    }

    fn convert<T: Convert>(&self, option: Option<T>) -> Option<String> {
        option.map(|value| value.convert(&self.config.units))
    }
}
