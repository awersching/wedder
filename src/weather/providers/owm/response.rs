use chrono::DateTime;
use chrono::Local;
use chrono::NaiveDateTime;
use chrono::offset::TimeZone;
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
    sunrise: i64,
    sunset: i64,
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

    fn kelvin_max(&self) -> f32 {
        self.main.temp_max
    }

    fn kelvin_min(&self) -> f32 {
        self.main.temp_min
    }

    fn pressure(&self) -> f32 {
        self.main.pressure
    }

    fn humidity(&self) -> f32 {
        self.main.humidity
    }

    fn wind_speed(&self) -> f32 {
        self.wind.speed
    }

    fn cloud_percentage(&self) -> f32 {
        self.clouds.all
    }

    fn sunrise(&self) -> DateTime<Local> {
        self.to_datetime(self.sys.sunrise)
    }

    fn sunset(&self) -> DateTime<Local> {
        self.to_datetime(self.sys.sunset)
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

    fn to_datetime(&self, unix_timestamp: i64) -> DateTime<Local> {
        let time = NaiveDateTime::from_timestamp(unix_timestamp, 0);
        Local.from_utc_datetime(&time)
    }
}
