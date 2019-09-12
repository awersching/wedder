use std::error::Error;
use std::thread;
use std::time::Duration;

use chrono::DateTime;
use chrono::Local;
use chrono::NaiveDateTime;
use chrono::offset::TimeZone;
use log::debug;
use log::error;

use crate::config;

pub type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

/// Returns the requested resource if available
/// If it is not available, loops every 5s and tries again until it succeeds
pub fn get_retry(url: &str) -> reqwest::Response {
    let mut result = reqwest::get(url);

    while let Err(err) = result {
        error!("{}", err.to_string());
        debug!("Retrying {} ...", url);
        println!("No internet");

        thread::sleep(Duration::from_secs(config::RETRY_TIMEOUT));
        result = reqwest::get(url);
    }
    result.unwrap()
}


pub fn to_datetime(unix_timestamp: i64) -> DateTime<Local> {
    let time = NaiveDateTime::from_timestamp(unix_timestamp, 0);
    Local.from_utc_datetime(&time)
}
