use std::fmt::Display;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process;

use directories::ProjectDirs;
use log::debug;
use log::error;
use log::warn;

use crate::APP_NAME;
use crate::config::Config;

pub fn default_config_path() -> Option<PathBuf> {
    let project = ProjectDirs::from(
        "rs",
        APP_NAME,
        APP_NAME,
    )?;
    Some([
        project.config_dir().to_str()?,
        &format!("{}.toml", APP_NAME),
    ].iter().collect())
}

pub fn load_config(path: &PathBuf) -> Option<Config> {
    debug!("Trying to open config file under {}", path.to_str()?);
    let cfg_str = match fs::read_to_string(path) {
        Ok(cfg_str) => Some(cfg_str),
        Err(err) => match err.kind() {
            io::ErrorKind::NotFound => None,
            _ => malformed_config(err)
        }
    };

    if cfg_str.is_none() {
        warn!("No config file found under {}, using defaults", path.to_str()?);
        return Some(Config::default());
    }
    match toml::from_str(&cfg_str?) {
        Ok(config) => Some(config),
        Err(err) => malformed_config(err)
    }
}

fn malformed_config<E: Display>(err: E) -> ! {
    error!("Error parsing config file: {}", err.to_string());
    println!("Malformed config file");
    process::exit(1)
}
