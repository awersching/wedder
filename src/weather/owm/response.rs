use chrono::Local;
use chrono::{DateTime, NaiveDateTime, TimeZone};
use log::warn;
use serde::Deserialize;

use crate::weather::weather_condition::WeatherCondition;
use crate::weather::{Hpa, Kelvin, Meter, Millimeter, Ms, Percentage, Weather};

#[derive(Debug, Deserialize)]
pub struct Response {
    weather: Vec<OwmWeather>,
    main: Main,
    visibility: Option<Meter>,
    wind: Option<Wind>,
    clouds: Option<Clouds>,
    rain: Option<Rain>,
    snow: Option<Snow>,
    sys: Option<Sys>,
}

#[derive(Debug, Deserialize)]
struct OwmWeather {
    id: i32,
}

#[derive(Debug, Deserialize)]
struct Main {
    temp: Kelvin,
    feels_like: Option<Kelvin>,
    temp_min: Option<Kelvin>,
    temp_max: Option<Kelvin>,
    pressure: Option<Hpa>,
    humidity: Option<Percentage>,
}

#[derive(Debug, Deserialize)]
struct Wind {
    speed: Option<Ms>,
}

#[derive(Debug, Deserialize)]
struct Clouds {
    all: Option<Percentage>,
}

#[derive(Debug, Deserialize)]
struct Rain {
    #[serde(rename = "1h")]
    last_hour: Option<Millimeter>,
}

#[derive(Debug, Deserialize)]
struct Snow {
    #[serde(rename = "1h")]
    last_hour: Option<Millimeter>,
}

#[derive(Debug, Deserialize)]
struct Sys {
    sunrise: Option<i64>,
    sunset: Option<i64>,
}

impl Weather for Response {
    fn weather_condition(&self) -> WeatherCondition {
        if self.weather.len() > 1 {
            self.combined_weather_condition(&self.weather)
        } else {
            self.weather_condition(&self.weather[0])
        }
    }

    fn temp(&self) -> Kelvin {
        self.main.temp
    }

    fn temp_feels_like(&self) -> Option<Kelvin> {
        self.main.feels_like
    }

    fn temp_max(&self) -> Option<Kelvin> {
        self.main.temp_max
    }

    fn temp_min(&self) -> Option<Kelvin> {
        self.main.temp_min
    }

    fn pressure(&self) -> Option<Hpa> {
        self.main.pressure
    }

    fn humidity(&self) -> Option<Percentage> {
        self.main.humidity
    }

    fn wind_speed(&self) -> Option<Ms> {
        self.wind.as_ref().and_then(|wind| wind.speed)
    }

    fn clouds(&self) -> Option<Percentage> {
        self.clouds.as_ref().and_then(|clouds| clouds.all)
    }

    fn visibility(&self) -> Option<Meter> {
        self.visibility
    }

    fn precipitation(&self) -> Option<Millimeter> {
        let rain = self.rain.as_ref().and_then(|rain| rain.last_hour);
        let snow = self.snow.as_ref().and_then(|snow| snow.last_hour);
        if let (Some(rain), Some(snow)) = (rain, snow) {
            Some(Millimeter(rain.0 + snow.0))
        } else if rain.is_some() {
            rain
        } else if snow.is_some() {
            snow
        } else {
            None
        }
    }

    fn sunrise(&self) -> Option<DateTime<Local>> {
        self.sys
            .as_ref()
            .and_then(|sys| sys.sunrise.map(to_datetime))
    }

    fn sunset(&self) -> Option<DateTime<Local>> {
        self.sys
            .as_ref()
            .and_then(|sys| sys.sunset.map(to_datetime))
    }
}

impl Response {
    fn weather_condition(&self, weather: &OwmWeather) -> WeatherCondition {
        let id = weather.id;
        let first_digit = if let Ok(digit) = id.to_string()[0..1].parse::<i32>() {
            digit
        } else {
            warn!("Couldn't parse weather id");
            return WeatherCondition::Unknown;
        };

        match first_digit {
            2 => WeatherCondition::Thunderstorm,

            // rain
            3 => match id {
                300 | 301 | 310 => WeatherCondition::Rain,
                302 | 311 | 312 | 313 | 314 | 321 => WeatherCondition::HeavyRain,
                _ => WeatherCondition::Unknown,
            },
            5 => match id {
                500 | 520 => WeatherCondition::Rain,
                501 | 502 | 503 | 504 | 511 | 521 | 522 | 531 => WeatherCondition::HeavyRain,
                _ => WeatherCondition::Unknown,
            },

            6 => WeatherCondition::Snow,
            7 => WeatherCondition::Mist,

            // clear sky and clouds
            8 => match id {
                800 => WeatherCondition::ClearSky,
                801 => WeatherCondition::FewClouds,
                802 => WeatherCondition::Clouds,
                803 | 804 => WeatherCondition::ManyClouds,
                _ => WeatherCondition::Unknown,
            },

            _ => WeatherCondition::Unknown,
        }
    }

    fn combined_weather_condition(&self, weather: &[OwmWeather]) -> WeatherCondition {
        let condition1 = self.weather_condition(&weather[0]);
        let condition2 = self.weather_condition(&weather[1]);

        let sun =
            condition1 == WeatherCondition::ClearSky || condition2 == WeatherCondition::ClearSky;
        let rain = condition1 == WeatherCondition::Rain
            || condition2 == WeatherCondition::Rain
            || condition1 == WeatherCondition::HeavyRain
            || condition2 == WeatherCondition::HeavyRain;
        if rain && sun {
            WeatherCondition::RainSun
        } else {
            condition1
        }
    }
}

fn to_datetime(unix_timestamp: i64) -> DateTime<Local> {
    let time = NaiveDateTime::from_timestamp(unix_timestamp, 0);
    Local.from_utc_datetime(&time)
}
