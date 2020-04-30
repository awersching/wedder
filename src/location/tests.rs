use crate::location::{CurrentLocation, Location};
use crate::location::ip_api::IpApi;
use crate::Result;

#[allow(dead_code)]
const URL: &str = "http://ip-api.com/json/24.48.0.1?fields=city,lat,lon";

impl IpApi {
    #[allow(dead_code)]
    fn mock(&self) -> Result<Location> {
        self.get(URL)
    }
}

#[cfg(test)]
mod tests {
    use crate::location::ip_api::IpApi;

    #[test]
    fn location() {
        let location = IpApi::new().mock();
        assert!(location.is_ok());
        let location = location.unwrap();

        assert_eq!("Montreal", location.city);
        assert_eq!(45.5808, location.lat);
        assert_eq!(-73.5825, location.lon);
    }
}
