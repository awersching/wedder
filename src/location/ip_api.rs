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

#[cfg(test)]
mod tests {
    use crate::location::{CurrentLocation, Location};
    use crate::location::ip_api::IpApi;
    use crate::Result;

    const URL: &str = "http://ip-api.com/json/24.48.0.1?fields=city,lat,lon";

    impl IpApi {
        fn mock(&self) -> Result<Location> {
            self.get(URL)
        }
    }

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
