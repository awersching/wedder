use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::config::Config;

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

    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Icons(HashMap<String, String>);

impl Icons {
    pub fn get(&self, condition: &str) -> Option<&String> {
        self.0.get(condition)
    }
}

impl Default for Icons {
    fn default() -> Self {
        let cfg_str = include_str!("../../examples/wedder.toml");
        let config: Config = toml::from_str(cfg_str).unwrap();
        config.icons
    }
}
