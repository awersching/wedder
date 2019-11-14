use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

use crate::config::LocationConfig;
use crate::location::ip_api::IpApi;
use crate::location::manual::Manual;
use crate::Result;

pub mod ip_api;
mod manual;

pub trait CurrentLocation {
    fn current_location(&self) -> Result<Location>;
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
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

impl LocationProvider {
    pub fn create(provider: &LocationConfig) -> Box<dyn CurrentLocation> {
        match provider.provider {
            Self::Ip => Box::new(IpApi::new()),
            Self::Manual => Box::new(Manual::new(&provider.location))
        }
    }
}

impl Default for LocationProvider {
    fn default() -> Self {
        Self::Ip
    }
}
