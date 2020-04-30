use serde::Deserialize;
use serde::Serialize;
use strum_macros::EnumString;

#[derive(Debug, Serialize, Deserialize, EnumString, Eq, PartialEq, Clone)]
pub enum Temperature {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl Default for Temperature {
    fn default() -> Self {
        Self::Celsius
    }
}

#[derive(Debug, Serialize, Deserialize, EnumString, Eq, PartialEq, Clone)]
pub enum WindSpeed {
    Ms,
    Kmh,
    Mph,
}

impl Default for WindSpeed {
    fn default() -> Self {
        Self::Kmh
    }
}
