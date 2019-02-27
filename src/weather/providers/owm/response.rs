use log::warn;
use serde::Deserialize;

use crate::weather;
use crate::weather::weather_condition::WeatherCondition;

#[derive(Debug, Deserialize)]
pub struct Response {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    clouds: Clouds,
    sys: Sys,
}

#[derive(Debug, Deserialize)]
struct Weather {
    id: i32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Debug, Deserialize)]
struct Main {
    temp: f32,
    pressure: f32,
    humidity: f32,
    temp_min: f32,
    temp_max: f32,
}

#[derive(Debug, Deserialize)]
struct Wind {
    speed: f32,
}

#[derive(Debug, Deserialize)]
struct Clouds {
    all: f32,
}

#[derive(Debug, Deserialize)]
struct Sys {
    sunrise: i32,
    sunset: i32,
}

impl weather::Weather for Response {
    fn weather_condition(&self) -> WeatherCondition {
        if self.weather.len() > 1 {
            self.combined_weather_condition(&self.weather)
        } else {
            self.weather_condition(&self.weather[0])
        }
    }

    fn kelvin(&self) -> f32 {
        self.main.temp
    }
}

impl Response {
    fn weather_condition(&self, weather: &Weather) -> WeatherCondition {
        let id = weather.id;
        let first_digit = match id.to_string()[0..1].parse::<i32>() {
            Ok(digit) => digit,
            Err(_) => {
                warn!("Couldn't parse weather id");
                return WeatherCondition::Unknown;
            }
        };

        match first_digit {
            2 => WeatherCondition::Thunderstorm,

            // rain
            3 => match id {
                300 | 301 | 310 => WeatherCondition::Rain,
                302 | 311 | 312 | 313 | 314 | 321 => WeatherCondition::HeavyRain,
                _ => WeatherCondition::Unknown
            },
            5 => match id {
                500 | 520 => WeatherCondition::Rain,
                501 | 502 | 503 | 504 | 511 | 521 | 522 | 531 => WeatherCondition::HeavyRain,
                _ => WeatherCondition::Unknown
            },

            6 => WeatherCondition::Snow,
            7 => WeatherCondition::Mist,

            // clear sky and clouds
            8 => match id {
                800 => WeatherCondition::ClearSky,
                801 => WeatherCondition::FewClouds,
                802 => WeatherCondition::Clouds,
                803 | 804 => WeatherCondition::ManyClouds,
                _ => WeatherCondition::Unknown
            },

            _ => WeatherCondition::Unknown
        }
    }

    fn combined_weather_condition(&self, weather: &[Weather]) -> WeatherCondition {
        let condition1 = self.weather_condition(&weather[0]);
        let condition2 = self.weather_condition(&weather[1]);

        let sun = condition1 == WeatherCondition::ClearSky ||
            condition2 == WeatherCondition::ClearSky;
        let rain = condition1 == WeatherCondition::Rain ||
            condition2 == WeatherCondition::Rain ||
            condition1 == WeatherCondition::HeavyRain ||
            condition2 == WeatherCondition::HeavyRain;

        if rain && sun {
            WeatherCondition::RainSun
        } else {
            condition1
        }
    }
}
