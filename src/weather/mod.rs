use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

use crate::config::{Distance, Precipitation, Temperature, WindSpeed};
use crate::location::Location;
use crate::weather::owm::{OpenWeatherMap, OwmMock};
use crate::weather::weather_condition::WeatherCondition;
use crate::Result;
use std::fmt;
use std::fmt::{Display, Formatter};

pub mod formatter;
mod owm;
pub mod weather_condition;

pub trait CurrentWeather {
    fn weather(&self, location: &Location, api_key: &str) -> Result<Box<dyn Weather>>;
}

pub trait Weather {
    fn weather_condition(&self) -> WeatherCondition;

    fn temp(&self) -> Kelvin;
    fn temp_feels_like(&self) -> Option<Kelvin>;
    fn temp_max(&self) -> Option<Kelvin>;
    fn temp_min(&self) -> Option<Kelvin>;

    fn pressure(&self) -> Option<Hpa>;
    fn humidity(&self) -> Option<Percentage>;
    fn wind_speed(&self) -> Option<Ms>;
    fn clouds(&self) -> Option<Percentage>;
    fn visibility(&self) -> Option<Meter>;
    fn precipitation(&self) -> Option<Millimeter>;

    fn sunrise(&self) -> Option<DateTime<Local>>;
    fn sunset(&self) -> Option<DateTime<Local>>;
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct Kelvin(f32);

impl Display for Kelvin {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Kelvin {
    fn convert(&self, unit: &Temperature) -> f32 {
        match unit {
            Temperature::Celsius => self.0 - 273.15,
            Temperature::Fahrenheit => (self.0 - 273.15) * (9.0 / 5.0) + 32.0,
            Temperature::Kelvin => self.0,
        }
        .round()
    }
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct Hpa(f32);

impl Display for Hpa {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct Percentage(f32);

impl Display for Percentage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct Ms(f32);

impl Display for Ms {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Ms {
    fn convert(&self, unit: &WindSpeed) -> f32 {
        match unit {
            WindSpeed::Ms => self.0,
            WindSpeed::Kmh => self.0 * 3.6,
            WindSpeed::Mph => self.0 * (3600.0 / 1609.34),
        }
        .round()
    }
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct Meter(f32);

impl Display for Meter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Meter {
    fn convert(&self, unit: &Distance) -> f32 {
        match unit {
            Distance::Meter => self.0,
            Distance::Kilometer => self.0 / 1000.0,
            Distance::Mile => self.0 * 0.000_621_371_2,
        }
        .round()
    }
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct Millimeter(f32);

impl Display for Millimeter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Millimeter {
    fn convert(&self, unit: &Precipitation) -> f32 {
        match unit {
            Precipitation::Millimeter => self.0,
            Precipitation::Inch => self.0 / 25.4,
        }
        .round()
    }
}

#[derive(Debug, Serialize, Deserialize, EnumString, Eq, PartialEq, Clone)]
pub enum WeatherProvider {
    OpenWeatherMap,
    OwmMock,
}

impl WeatherProvider {
    pub fn create(provider: &Self) -> Box<dyn CurrentWeather> {
        match provider {
            Self::OpenWeatherMap => Box::new(OpenWeatherMap::new()),
            Self::OwmMock => Box::new(OwmMock::new()),
        }
    }
}

impl Default for WeatherProvider {
    fn default() -> Self {
        Self::OpenWeatherMap
    }
}
