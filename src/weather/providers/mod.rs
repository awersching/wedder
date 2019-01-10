use std::thread;
use std::time::Duration;

use serde::Deserialize;
use serde::Serialize;

pub mod owm;

trait Get {
    /// Returns the requested resource if available
    /// If it is not available, loops every 5s and tries again until it succeeds
    fn get(&self, url: &str) -> reqwest::Response {
        let mut result = reqwest::get(url);
        while result.is_err() {
            println!("No connection");
            thread::sleep(Duration::from_secs(5));
            result = reqwest::get(url);
        }
        result.unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub enum WeatherProvider {
    OpenWeatherMap
}
