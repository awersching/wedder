use std::error::Error;

use serde::{Deserialize, Serialize};

pub mod ip_api;

pub trait CurrentLocation {
    fn current_location(&self) -> Result<Location, Box<dyn Error>>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    #[serde(default)]
    pub lat: f32,
    #[serde(default)]
    pub lon: f32,
}

#[derive(Serialize, Deserialize)]
pub enum LocationProvider {
    Ip,
    Manual,
}
