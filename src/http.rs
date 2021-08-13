use std::time::Duration;
use std::{process, thread};

use log::debug;
use log::error;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

const RETRY_TIMEOUT: u64 = 15;

pub fn get<T: Debug + DeserializeOwned>(url: &str) -> crate::Result<T> {
    debug!("Querying {} ...", url);
    let mut result = reqwest::blocking::get(url);
    while let Err(err) = result {
        println!("Unavailable");
        error!("{}", err.to_string());
        debug!("Retrying {} ...", url);
        thread::sleep(Duration::from_secs(RETRY_TIMEOUT));
        result = reqwest::blocking::get(url);
    }

    let response = result?;
    debug!("HTTP {}", response.status().to_string());
    if response.status().as_u16() == 401 {
        println!("Invalid/unauthorized API key");
        process::exit(1)
    }
    let t: T = response.json()?;
    debug!("{:#?}", t);
    Ok(t)
}
