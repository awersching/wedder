use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Hash, Eq, PartialEq, Serialize, Deserialize, Display)]
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

    icons.insert(WeatherCondition::ClearSky.to_string(), "".to_string());
    icons.insert(WeatherCondition::FewClouds.to_string(), "".to_string());
    icons.insert(WeatherCondition::Clouds.to_string(), "摒".to_string());
    icons.insert(WeatherCondition::ManyClouds.to_string(), "".to_string());
    icons.insert(WeatherCondition::RainSun.to_string(), "".to_string());
    icons.insert(WeatherCondition::Rain.to_string(), "".to_string());
    icons.insert(WeatherCondition::HeavyRain.to_string(), "歹".to_string());
    icons.insert(WeatherCondition::Thunderstorm.to_string(), "".to_string());
    icons.insert(WeatherCondition::Snow.to_string(), "".to_string());
    icons.insert(WeatherCondition::Mist.to_string(), "".to_string());

    icons
}
