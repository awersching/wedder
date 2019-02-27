use log::debug;

use crate::location::CurrentLocation;
use crate::location::Location;
use crate::util;
use crate::util::Result;

pub struct IpApi;

const URL: &str = "http://ip-api.com/json/?fields=city,lat,lon";

impl CurrentLocation for IpApi {
    fn current_location(&self) -> Result<Location> {
        debug!("Querying {} ...", URL);
        let mut response = util::get_retry(URL);
        debug!("HTTP {}", response.status().to_string());

        let location: Location = serde_json::from_str(&response.text()?)?;
        Ok(location)
    }
}

impl IpApi {
    pub fn new() -> Self {
        IpApi
    }
}
