use std::error::Error;

use crate::location::CurrentLocation;
use crate::location::Location;
use crate::util;

pub struct IpApi {}

impl CurrentLocation for IpApi {
    fn current_location(&self) -> Result<Location, Box<dyn Error>> {
        let url = "http://ip-api.com/json/?fields=lat,lon";
        let body = util::get_retry(url, "No location").text()?;

        let location: Location = serde_json::from_str(&body)?;
        Ok(location)
    }
}

impl IpApi {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> impl CurrentLocation {
        IpApi {}
    }
}
