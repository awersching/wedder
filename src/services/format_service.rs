use std::collections::HashMap;

use crate::model::config::Config;
use crate::model::location::Location;
use crate::model::weather::Convert;
use crate::model::weather::Weather;

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

pub struct FormatService<'a> {
    config: &'a Config,
    location: Location,
    weather: Box<dyn Weather>,
}

impl<'a> FormatService<'a> {
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
        let temp_unit = &self.config.units.temperature;

        let city = &self.location.city;
        let icon = self.icon();

        let temperature = self.weather.temp().map(|temp| temp.convert(temp_unit));
        let temperature_feels_like = self
            .weather
            .temp_feels_like()
            .map(|feels_like| feels_like.convert(temp_unit));
        let temperature_max = self.weather.temp_max().map(|max| max.convert(temp_unit));
        let temperature_min = self.weather.temp_min().map(|min| min.convert(temp_unit));
        let dew_point = self
            .weather
            .dew_point()
            .map(|dew_point| dew_point.convert(temp_unit));

        let precipitation = self
            .weather
            .precipitation()
            .map(|precipitation| precipitation.convert(&self.config.units.precipitation));
        let precipitation_chance = self.weather.precipitation_chance();
        let clouds = self.weather.clouds();
        let humidity = self.weather.humidity();
        let visibility = self
            .weather
            .visibility()
            .map(|visibility| visibility.convert(&self.config.units.distance));
        let wind_speed = self
            .weather
            .wind_speed()
            .map(|wind_speed| wind_speed.convert(&self.config.units.wind_speed));
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
}
