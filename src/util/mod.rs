use std::thread;
use std::time::Duration;

use crate::config;

/// Returns the requested resource if available
/// If it is not available, loops every 5s and tries again until it succeeds
pub fn get_retry(url: &str, retry_message: &str) -> reqwest::Response {
    let mut result = reqwest::get(url);
    while result.is_err() {
        println!("{}", retry_message);
        thread::sleep(Duration::from_secs(config::RETRY_TIMEOUT));
        result = reqwest::get(url);
    }
    result.unwrap()
}
