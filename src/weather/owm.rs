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
        let one_call_url = format!(
            "{}lat={}&lon={}&APPID={}",
            OneCall::URL,
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

        let one_call = http::get(&one_call_url).ok();
        let air_pollution = http::get(&air_pollution_url).ok();
        Ok(Box::new(OwmWeather::new(one_call, air_pollution)))
    }
}

#[derive(Debug)]
struct OwmWeather {
    one_call: Option<OneCall>,
    air_pollution: Option<AirPollution>,
}

impl OwmWeather {
    fn new(one_call: Option<OneCall>, air_pollution: Option<AirPollution>) -> Self {
        Self {
            one_call,
            air_pollution,
        }
    }
}

impl Weather for OwmWeather {
    fn weather_condition(&self) -> Option<WeatherCondition> {
        let conditions = self.one_call.as_ref()?.current.weather.as_ref()?;
        let icon = &*conditions.get(0)?.icon;
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
        self.one_call.as_ref()?.current.temp
    }

    fn temp_feels_like(&self) -> Option<Kelvin> {
        self.one_call.as_ref()?.current.feels_like
    }

    fn temp_max(&self) -> Option<Kelvin> {
        self.one_call.as_ref()?.daily.get(0)?.temp.max
    }

    fn temp_min(&self) -> Option<Kelvin> {
        self.one_call.as_ref()?.daily.get(0)?.temp.min
    }

    fn dew_point(&self) -> Option<Kelvin> {
        self.one_call.as_ref()?.current.dew_point
    }

    fn precipitation(&self) -> Option<Millimeter> {
        let current = &self.one_call.as_ref()?.current;
        let rain = current.rain.as_ref().map(|rain| rain.last_hour);
        let snow = current.snow.as_ref().map(|snow| snow.last_hour);
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
        let pop = self.one_call.as_ref()?.hourly.get(0)?.pop;
        pop.map(|pop| Percentage(pop.0 * 100.0))
    }

    fn clouds(&self) -> Option<Percentage> {
        self.one_call.as_ref()?.current.clouds
    }

    fn humidity(&self) -> Option<Percentage> {
        self.one_call.as_ref()?.current.humidity
    }

    fn visibility(&self) -> Option<Meter> {
        self.one_call.as_ref()?.current.visibility
    }

    fn wind_speed(&self) -> Option<Ms> {
        self.one_call.as_ref()?.current.wind_speed
    }

    fn pressure(&self) -> Option<Hpa> {
        self.one_call.as_ref()?.current.pressure
    }

    fn uvi(&self) -> Option<Uvi> {
        self.one_call.as_ref()?.current.uvi
    }

    fn aqi(&self) -> Option<Aqi> {
        self.air_pollution.as_ref()?.list.get(0)?.main.as_ref()?.aqi
    }

    fn sunrise(&self) -> Option<DateTime<Local>> {
        self.one_call.as_ref()?.current.sunrise.map(to_datetime)
    }

    fn sunset(&self) -> Option<DateTime<Local>> {
        self.one_call.as_ref()?.current.sunset.map(to_datetime)
    }
}

#[derive(Deserialize)]
struct OneCall {
    current: Current,
    hourly: Vec<Hourly>,
    daily: Vec<Daily>,
}

impl OneCall {
    const URL: &'static str = "http://api.openweathermap.org/data/2.5/onecall?\
        exclude=minutely,alerts&";
}

impl Debug for OneCall {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("OneCall")
            .field("current", &self.current)
            .field("hourly", &self.hourly.get(0))
            .field("daily", &self.daily.get(0))
            .finish()
    }
}

#[derive(Debug, Deserialize)]
struct Current {
    sunrise: Option<i64>,
    sunset: Option<i64>,
    temp: Option<Kelvin>,
    feels_like: Option<Kelvin>,
    pressure: Option<Hpa>,
    humidity: Option<Percentage>,
    dew_point: Option<Kelvin>,
    uvi: Option<Uvi>,
    clouds: Option<Percentage>,
    visibility: Option<Meter>,
    wind_speed: Option<Ms>,
    rain: Option<Rain>,
    snow: Option<Snow>,
    weather: Option<Vec<OwmWeatherCondition>>,
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

#[derive(Debug, Deserialize)]
struct Hourly {
    pop: Option<Percentage>,
}

#[derive(Debug, Deserialize)]
struct Daily {
    temp: Temp,
}

#[derive(Debug, Deserialize)]
struct Temp {
    min: Option<Kelvin>,
    max: Option<Kelvin>,
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
            .field("list", &self.list.get(0))
            .finish()
    }
}

#[derive(Debug, Deserialize)]
struct ListItem {
    main: Option<Main>,
}

#[derive(Debug, Deserialize)]
struct Main {
    aqi: Option<Aqi>,
}

fn to_datetime(unix_timestamp: i64) -> DateTime<Local> {
    let time = NaiveDateTime::from_timestamp(unix_timestamp, 0);
    Local.from_utc_datetime(&time)
}

#[cfg(feature = "test")]
impl CurrentWeather for OpenWeatherMap {
    fn weather(&self, location: &Location, api_key: &str) -> crate::Result<Box<dyn Weather>> {
        let one_call_str = include_str!("../../tests/one_call.json");
        let air_pollution_str = include_str!("../../tests/air_pollution.json");
        let one_call: Option<OneCall> = serde_json::from_str(one_call_str).ok();
        let air_pollution: Option<AirPollution> = serde_json::from_str(air_pollution_str).ok();
        Ok(Box::new(OwmWeather::new(one_call, air_pollution)))
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
    assert_eq!(weather.temp().unwrap().0, 274.75);
    assert_eq!(weather.temp_feels_like().unwrap().0, 270.4);
    assert_eq!(weather.temp_max().unwrap().0, 279.4);
    assert_eq!(weather.temp_min().unwrap().0, 273.15);
    assert_eq!(weather.dew_point().unwrap().0, 274.18);
    assert_eq!(weather.precipitation().unwrap().0, 0.074);
    assert_eq!(weather.precipitation_chance().unwrap().0, 10.0);
    assert_eq!(weather.clouds().unwrap().0, 90.0);
    assert_eq!(weather.humidity().unwrap().0, 96.0);
    assert_eq!(weather.visibility().unwrap().0, 6437.0);
    assert_eq!(weather.wind_speed().unwrap().0, 3.6);
    assert_eq!(weather.pressure().unwrap().0, 1017.0);
    assert_eq!(weather.uvi().unwrap().0, 0.0);
    assert_eq!(weather.aqi().unwrap().0, 1.0);
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
