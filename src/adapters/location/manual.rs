use crate::model::location::{CurrentLocation, Location};

pub struct Manual {
    location: Location,
}

impl Manual {
    pub fn new(location: &Location) -> Self {
        Self {
            location: location.clone(),
        }
    }
}

impl CurrentLocation for Manual {
    fn location(&self) -> crate::Result<Location> {
        Ok(self.location.clone())
    }
}
