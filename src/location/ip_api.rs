use std::error::Error;

use serde::Deserialize;

use crate::location::Coordinates;
use crate::location::CurrentLocation;
use crate::location::Location;
use crate::util;

pub struct IpApi {}

impl CurrentLocation for IpApi {
    fn current_location(&self) -> Result<Location, Box<dyn Error>> {
        let url = "http://ip-api.com/json/";
        let body = util::get_retry(url, "No location")
            .text().unwrap();
        let response: Response = serde_json::from_str(&body)?;

        let coordinates = Coordinates {
            lat: response.lat,
            lon: response.lon,
        };
        Ok(Location {
            city: Some(response.city),
            coordinates: Some(coordinates),
        })
    }
}

impl IpApi {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> impl CurrentLocation {
        IpApi {}
    }
}

#[derive(Deserialize)]
struct Response {
    city: String,
    lat: f32,
    lon: f32,
}
