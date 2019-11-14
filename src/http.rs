use std::thread;
use std::time::Duration;

use log::debug;
use log::error;

const RETRY_TIMEOUT: u64 = 15;

/// Returns the requested resource if available
/// If it is not available, loops every 5s and tries again until it succeeds
pub fn get_retry(url: &str) -> reqwest::Response {
    let mut result = reqwest::get(url);

    while let Err(err) = result {
        error!("{}", err.to_string());
        debug!("Retrying {} ...", url);
        println!("No internet");

        thread::sleep(Duration::from_secs(RETRY_TIMEOUT));
        result = reqwest::get(url);
    }
    result.unwrap()
}
