use crate::location::{CurrentLocation, Location};
use crate::util::Result;

pub struct Manual {
    location: Location
}

impl CurrentLocation for Manual {
    fn current_location(&self) -> Result<Location> {
        Ok(self.location.clone())
    }
}

impl Manual {
    pub fn new(location: &Location) -> Self {
        Self {
            location: location.clone()
        }
    }
}
