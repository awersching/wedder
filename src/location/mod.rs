use std::error::Error;

pub mod geoclue;
pub mod ip_api;

pub trait CurrentLocation {
    fn current_location(&self) -> Result<Location, Box<dyn Error>>;
}

#[derive(Debug)]
pub struct Location {
    pub city: Option<String>,
    pub coordinates: Option<Coordinates>,
}

#[derive(Debug)]
pub struct Coordinates {
    pub lat: f32,
    pub lon: f32,
}
