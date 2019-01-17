use log::debug;

use crate::location::CurrentLocation;
use crate::location::Location;
use crate::util;

pub struct IpApi {}

impl CurrentLocation for IpApi {
    fn current_location(&self) -> util::Result<Location> {
        let url = "http://ip-api.com/json/?fields=city,lat,lon";
        debug!("Querying {}...", url);
        let body = util::get_retry(url).text()?;

        let location: Location = serde_json::from_str(&body)?;
        Ok(location)
    }
}

impl IpApi {
    pub fn new() -> Self {
        IpApi {}
    }
}
