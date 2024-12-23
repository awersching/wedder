use crate::{
    adapters::location::{ip_api::IpApi, manual::Manual},
    model::{
        config::LocationConfig,
        location::{CurrentLocation, LocationProvider},
    },
};

pub fn current_location(provider: &LocationConfig) -> Box<dyn CurrentLocation> {
    match provider.provider {
        LocationProvider::IpApi => Box::new(IpApi::new()),
        LocationProvider::Manual => Box::new(Manual::new(&provider.location)),
    }
}
