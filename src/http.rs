use std::thread;
use std::time::Duration;

use log::debug;
use log::error;

const RETRY_TIMEOUT: u64 = 15;

pub fn get_retry(url: &str) -> reqwest::blocking::Response {
    let mut result = reqwest::blocking::get(url);

    while let Err(err) = result {
        error!("{}", err.to_string());
        debug!("Retrying {} ...", url);
        println!("No internet");

        thread::sleep(Duration::from_secs(RETRY_TIMEOUT));
        result = reqwest::blocking::get(url);
    }
    result.unwrap()
}
