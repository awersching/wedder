use std::collections::HashMap;

use crate::config::Config;
use crate::location::Location;
use crate::weather::Weather;

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

        let city = Some(&self.location.city);
        let icon = Some(self.icon());
        let temperature = Some(self.weather.temp().convert(&self.config.units.temperature));
        let temperature_feels_like = self
            .weather
            .temp_feels_like()
            .map(|kelvin| kelvin.convert(&self.config.units.temperature));
        let temperature_max = self
            .weather
            .temp_max()
            .map(|kelvin| kelvin.convert(&self.config.units.temperature));
        let temperature_min = self
            .weather
            .temp_min()
            .map(|kelvin| kelvin.convert(&self.config.units.temperature));
        let pressure = self.weather.pressure();
        let humidity = self.weather.humidity();
        let wind_speed = self
            .weather
            .wind_speed()
            .map(|ms| ms.convert(&self.config.units.wind_speed));
        let cloud_percentage = self.weather.clouds();
        let visibility = self.weather.visibility();
        let precipitation = self.weather.precipitation();
        let sunrise = self.weather.sunrise().map(|time| time.format("%H:%M"));
        let sunset = self.weather.sunset().map(|time| time.format("%H:%M"));

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
        tag!(tags, visibility);
        tag!(tags, precipitation);
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
}
