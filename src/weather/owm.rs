use crate::http;
use crate::location::Location;
use crate::weather::weather_condition::WeatherCondition;
use crate::weather::Weather;
use crate::weather::{Aqi, CurrentWeather, Hpa, Kelvin, Meter, Millimeter, Ms, Percentage, Uvi};
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use serde::Deserialize;
use std::fmt;
use std::fmt::{Debug, Formatter};

pub struct OpenWeatherMap;

impl OpenWeatherMap {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(not(feature = "test"))]
impl CurrentWeather for OpenWeatherMap {
    fn weather(&self, location: &Location, api_key: &str) -> crate::Result<Box<dyn Weather>> {
        let current_weather_url = format!(
            "{}lat={}&lon={}&APPID={}",
            Current::URL,
            location.lat,
            location.lon,
            api_key
        );
        let air_pollution_url = format!(
            "{}lat={}&lon={}&APPID={}",
            AirPollution::URL,
            location.lat,
            location.lon,
            api_key
        );

        let current = http::get(&current_weather_url).ok();
        let air_pollution = http::get(&air_pollution_url).ok();
        Ok(Box::new(OwmWeather::new(current, air_pollution)))
    }
}

#[derive(Debug)]
struct OwmWeather {
    current: Option<Current>,
    air_pollution: Option<AirPollution>,
}

impl OwmWeather {
    fn new(current: Option<Current>, air_pollution: Option<AirPollution>) -> Self {
        Self {
            current,
            air_pollution,
        }
    }
}

impl Weather for OwmWeather {
    fn weather_condition(&self) -> Option<WeatherCondition> {
        let conditions = self.current.as_ref()?.weather.as_ref()?;
        let icon = &*conditions.first()?.icon;
        match icon {
            "01d" | "01n" => Some(WeatherCondition::ClearSky),
            "02d" | "02n" => Some(WeatherCondition::FewClouds),
            "03d" | "03n" => Some(WeatherCondition::Clouds),
            "04d" | "04n" => Some(WeatherCondition::ManyClouds),
            "09d" | "09n" => Some(WeatherCondition::Rain),
            "10d" | "10n" => Some(WeatherCondition::HeavyRain),
            "11d" | "11n" => Some(WeatherCondition::Thunderstorm),
            "13d" | "13n" => Some(WeatherCondition::Snow),
            "50d" | "50n" => Some(WeatherCondition::Mist),
            _ => None,
        }
    }

    fn temp(&self) -> Option<Kelvin> {
        self.current.as_ref()?.main.as_ref()?.temp
    }

    fn temp_feels_like(&self) -> Option<Kelvin> {
        self.current.as_ref()?.main.as_ref()?.feels_like
    }

    fn temp_max(&self) -> Option<Kelvin> {
        self.current.as_ref()?.main.as_ref()?.temp_max
    }

    fn temp_min(&self) -> Option<Kelvin> {
        self.current.as_ref()?.main.as_ref()?.temp_min
    }

    fn dew_point(&self) -> Option<Kelvin> {
        None
    }

    fn precipitation(&self) -> Option<Millimeter> {
        let rain = self
            .current
            .as_ref()?
            .rain
            .as_ref()
            .map(|rain| rain.last_hour);
        let snow = self
            .current
            .as_ref()?
            .snow
            .as_ref()
            .map(|snow| snow.last_hour);
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

    fn precipitation_chance(&self) -> Option<Percentage> {
        None
    }

    fn clouds(&self) -> Option<Percentage> {
        self.current.as_ref()?.clouds.as_ref()?.all
    }

    fn humidity(&self) -> Option<Percentage> {
        self.current.as_ref()?.main.as_ref()?.humidity
    }

    fn visibility(&self) -> Option<Meter> {
        self.current.as_ref()?.visibility
    }

    fn wind_speed(&self) -> Option<Ms> {
        self.current.as_ref()?.wind.as_ref()?.speed
    }

    fn pressure(&self) -> Option<Hpa> {
        self.current.as_ref()?.main.as_ref()?.pressure
    }

    fn uvi(&self) -> Option<Uvi> {
        None
    }

    fn aqi(&self) -> Option<Aqi> {
        self.air_pollution
            .as_ref()?
            .list
            .first()?
            .main
            .as_ref()?
            .aqi
    }

    fn sunrise(&self) -> Option<DateTime<Local>> {
        self.current
            .as_ref()?
            .sys
            .as_ref()?
            .sunrise
            .map(to_datetime)
    }

    fn sunset(&self) -> Option<DateTime<Local>> {
        self.current.as_ref()?.sys.as_ref()?.sunset.map(to_datetime)
    }
}

#[derive(Debug, Deserialize)]
struct Current {
    weather: Option<Vec<OwmWeatherCondition>>,
    main: Option<CurrentMain>,
    visibility: Option<Meter>,
    wind: Option<Wind>,
    rain: Option<Rain>,
    snow: Option<Snow>,
    clouds: Option<Clouds>,
    sys: Option<Sys>,
}

impl Current {
    const URL: &'static str = "http://api.openweathermap.org/data/2.5/weather?";
}

#[derive(Debug, Deserialize)]
struct CurrentMain {
    temp: Option<Kelvin>,
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
struct Sys {
    sunrise: Option<i64>,
    sunset: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct OwmWeatherCondition {
    icon: String,
}

#[derive(Debug, Deserialize)]
struct Rain {
    #[serde(rename = "1h")]
    last_hour: Millimeter,
}

#[derive(Debug, Deserialize)]
struct Snow {
    #[serde(rename = "1h")]
    last_hour: Millimeter,
}

#[derive(Deserialize)]
struct AirPollution {
    list: Vec<ListItem>,
}

impl AirPollution {
    const URL: &'static str = "http://api.openweathermap.org/data/2.5/air_pollution?";
}

impl Debug for AirPollution {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("AirPollution")
            .field("list", &self.list.first())
            .finish()
    }
}

#[derive(Debug, Deserialize)]
struct ListItem {
    main: Option<AirPollutionMain>,
}

#[derive(Debug, Deserialize)]
struct AirPollutionMain {
    aqi: Option<Aqi>,
}

fn to_datetime(unix_timestamp: i64) -> DateTime<Local> {
    let time = NaiveDateTime::from_timestamp(unix_timestamp, 0);
    Local.from_utc_datetime(&time)
}

#[cfg(feature = "test")]
impl CurrentWeather for OpenWeatherMap {
    fn weather(&self, location: &Location, api_key: &str) -> crate::Result<Box<dyn Weather>> {
        let current_str = include_str!("../../tests/current.json");
        let air_pollution_str = include_str!("../../tests/air_pollution.json");
        let current: Option<Current> = serde_json::from_str(current_str).ok();
        let air_pollution: Option<AirPollution> = serde_json::from_str(air_pollution_str).ok();
        Ok(Box::new(OwmWeather::new(current, air_pollution)))
    }
}

#[test]
fn weather() {
    let location = Location {
        city: None,
        lat: 0.0,
        lon: 0.0,
    };
    let weather = OpenWeatherMap::new().weather(&location, "");
    assert!(weather.is_ok());
    let weather = weather.unwrap();

    assert_eq!(weather.weather_condition().unwrap(), WeatherCondition::Mist);
    assert_eq!(weather.temp().unwrap().0, 274.753);
    assert_eq!(weather.temp_feels_like().unwrap().0, 270.456);
    assert_eq!(weather.temp_max().unwrap().0, 279.471);
    assert_eq!(weather.temp_min().unwrap().0, 273.159);
    assert_eq!(weather.dew_point(), None);
    assert_eq!(weather.precipitation().unwrap().0, 0.074);
    assert_eq!(weather.precipitation_chance(), None);
    assert_eq!(weather.clouds().unwrap().0, 90.563);
    assert_eq!(weather.humidity().unwrap().0, 96.775);
    assert_eq!(weather.visibility().unwrap().0, 6437.888);
    assert_eq!(weather.wind_speed().unwrap().0, 3.654);
    assert_eq!(weather.pressure().unwrap().0, 1017.567);
    assert_eq!(weather.uvi(), None);
    assert_eq!(weather.aqi().unwrap().0, 1.33);
    assert!(weather.sunrise().is_some());
    let sunrise = DateTime::<chrono::Utc>::from(weather.sunrise().unwrap())
        .format("%H:%M")
        .to_string();
    assert_eq!(sunrise, "13:13");
    assert!(weather.sunset().is_some());
    let sunset = DateTime::<chrono::Utc>::from(weather.sunset().unwrap())
        .format("%H:%M")
        .to_string();
    assert_eq!(sunset, "23:10");
}
