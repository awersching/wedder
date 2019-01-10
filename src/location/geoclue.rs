use std::error::Error;

use crate::location::CurrentLocation;
use crate::location::Location;

#[allow(dead_code)]
pub struct Geoclue {}

impl CurrentLocation for Geoclue {
    fn current_location(&self) -> Result<Location, Box<dyn Error>> {
        unimplemented!()
    }
}
