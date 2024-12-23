use crate::{
    adapters::http,
    model::location::{CurrentLocation, Location},
};

pub struct IpApi;

impl IpApi {
    const URL: &'static str = "http://ip-api.com/json/?fields=city,lat,lon";
    pub fn new() -> Self {
        Self
    }
}

#[cfg(not(feature = "test"))]
impl CurrentLocation for IpApi {
    fn location(&self) -> crate::Result<Location> {
        http::get(Self::URL)
    }
}

#[cfg(feature = "test")]
impl CurrentLocation for IpApi {
    fn location(&self) -> crate::Result<Location> {
        http::get("http://ip-api.com/json/24.48.0.1?fields=city,lat,lon")
    }
}

#[test]
fn location() {
    let location = IpApi::new().location();
    assert!(location.is_ok());
    let location = location.unwrap();

    assert_eq!("Montreal", location.city.unwrap());
    assert_eq!(45.6026, location.lat);
    assert_eq!(-73.5167, location.lon);
}
