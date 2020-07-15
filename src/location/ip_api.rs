use crate::location::CurrentLocation;
use crate::location::Location;
use crate::Result;

pub struct IpApi;

const URL: &str = "http://ip-api.com/json/?fields=city,lat,lon";
const MOCK_URL: &str = "http://ip-api.com/json/24.48.0.1?fields=city,lat,lon";

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

pub struct IpApiMock;

impl IpApiMock {
    pub fn new() -> Self {
        Self
    }
}

impl CurrentLocation for IpApiMock {
    fn location(&self) -> Result<Location> {
        self.get(MOCK_URL)
    }
}

#[cfg(test)]
mod tests {
    use crate::location::CurrentLocation;
    use crate::location::ip_api::IpApiMock;

    #[test]
    fn location() {
        let location = IpApiMock::new().location();
        assert!(location.is_ok());
        let location = location.unwrap();

        assert_eq!("Montreal", location.city);
        assert_eq!(45.5808, location.lat);
        assert_eq!(-73.5825, location.lon);
    }
}
