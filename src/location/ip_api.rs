use log::debug;

use crate::location::CurrentLocation;
use crate::location::Location;
use crate::util;

pub struct IpApi;

const URL: &str = "http://ip-api.com/json/?fields=city,lat,lon";

impl CurrentLocation for IpApi {
    fn current_location(&self) -> util::Result<Location> {
        debug!("Querying {}...", URL);
        let body = util::get_retry(URL).text()?;

        let location: Location = serde_json::from_str(&body)?;
        Ok(location)
    }
}

impl IpApi {
    pub fn new() -> Self {
        IpApi
    }
}
