use log::debug;
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

use crate::config::LocationConfig;
use crate::http::get_retry;
use crate::location::ip_api::{IpApi, IpApiMock};
use crate::location::manual::Manual;
use crate::Result;

mod ip_api;
mod manual;

pub trait CurrentLocation {
    fn location(&self) -> Result<Location>;

    fn get(&self, url: &str) -> Result<Location> {
        debug!("Querying {} ...", url);
        let response = get_retry(url);
        debug!("HTTP {}", response.status().to_string());

        let location: Location = serde_json::from_str(&response.text()?)?;
        Ok(location)
    }
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

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.city == other.city &&
            (self.lat / 1e-7) as i32 == (other.lat / 1e-7) as i32 &&
            (self.lon / 1e-7) as i32 == (other.lon / 1e-7) as i32
    }
}

impl Eq for Location {}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, EnumString, Clone)]
pub enum LocationProvider {
    IpApi,
    IpApiMock,
    Manual,
}

impl LocationProvider {
    pub fn create(provider: &LocationConfig) -> Box<dyn CurrentLocation> {
        match provider.provider {
            Self::IpApi => Box::new(IpApi::new()),
            Self::IpApiMock => Box::new(IpApiMock::new()),
            Self::Manual => Box::new(Manual::new(&provider.location))
        }
    }
}

impl Default for LocationProvider {
    fn default() -> Self {
        Self::IpApi
    }
}
