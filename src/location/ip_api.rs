use crate::location::CurrentLocation;
use crate::location::Location;
use crate::Result;

pub struct IpApi;

const URL: &str = "http://ip-api.com/json/?fields=city,lat,lon";

impl CurrentLocation for IpApi {
    fn location(&self) -> Result<Location> {
        self.get(URL)
    }
}

impl IpApi {
    pub fn new() -> Self {
        Self
    }
}
