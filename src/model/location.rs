use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

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

impl Default for LocationProvider {
    fn default() -> Self {
        Self::IpApi
    }
}
