use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Hash, Eq, PartialEq, Serialize, Deserialize, Display)]
#[strum(serialize_all = "snake_case")]
pub enum WeatherCondition {
    ClearSky,
    FewClouds,
    Clouds,
    ManyClouds,
    RainSun,
    Rain,
    HeavyRain,
    Thunderstorm,
    Snow,
    Mist,
}

pub fn default_icons() -> HashMap<String, String> {
    let mut icons = HashMap::new();

    icons.insert(WeatherCondition::ClearSky, "");
    icons.insert(WeatherCondition::FewClouds, "");
    icons.insert(WeatherCondition::Clouds, "");
    icons.insert(WeatherCondition::ManyClouds, "");
    icons.insert(WeatherCondition::RainSun, "");
    icons.insert(WeatherCondition::Rain, "");
    icons.insert(WeatherCondition::HeavyRain, "");
    icons.insert(WeatherCondition::Thunderstorm, "");
    icons.insert(WeatherCondition::Snow, "");
    icons.insert(WeatherCondition::Mist, "");

    icons.into_iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect()
}
