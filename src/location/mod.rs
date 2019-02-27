use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

use crate::util::Result;

pub mod ip_api;

pub trait CurrentLocation {
    fn current_location(&self) -> Result<Location>;
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Location {
    #[serde(default)]
    pub city: String,
    #[serde(default)]
    pub lat: f32,
    #[serde(default)]
    pub lon: f32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, EnumString)]
pub enum LocationProvider {
    Ip,
    Manual,
}

impl Default for LocationProvider {
    fn default() -> Self {
        LocationProvider::Ip
    }
}
