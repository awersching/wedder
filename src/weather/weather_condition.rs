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
