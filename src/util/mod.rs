use std::error::Error;
use std::thread;
use std::time::Duration;

use log::debug;
use log::error;

use crate::config;

pub type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

/// Returns the requested resource if available
/// If it is not available, loops every 5s and tries again until it succeeds
pub fn get_retry(url: &str, retry_message: &str) -> reqwest::Response {
    let mut result = reqwest::get(url);

    while let Err(err) = result {
        error!("{}", err.to_string());
        println!("{}", retry_message);
        debug!("An error occurred getting {}, retrying...", url);

        thread::sleep(Duration::from_secs(config::RETRY_TIMEOUT));
        result = reqwest::get(url);
    }
    result.unwrap()
}
