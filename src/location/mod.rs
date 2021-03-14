use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

use crate::config::LocationConfig;
use crate::location::ip_api::IpApi;
use crate::location::manual::Manual;

mod ip_api;
mod manual;

pub trait CurrentLocation {
    fn location(&self) -> crate::Result<Location>;
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Location {
    pub city: Option<String>,
    #[serde(default)]
    pub lat: f32,
    #[serde(default)]
    pub lon: f32,
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.city == other.city
            && (self.lat / 1e-7) as i32 == (other.lat / 1e-7) as i32
            && (self.lon / 1e-7) as i32 == (other.lon / 1e-7) as i32
    }
}

impl Eq for Location {}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, EnumString, Clone)]
pub enum LocationProvider {
    IpApi,
    Manual,
}

impl LocationProvider {
    pub fn create(provider: &LocationConfig) -> Box<dyn CurrentLocation> {
        match provider.provider {
            Self::IpApi => Box::new(IpApi::new()),
            Self::Manual => Box::new(Manual::new(&provider.location)),
        }
    }
}

impl Default for LocationProvider {
    fn default() -> Self {
        Self::IpApi
    }
}
