use std::fmt::Display;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process;

use directories::ProjectDirs;
use log::debug;
use log::error;
use log::warn;

use crate::config::Config;
use crate::APP_NAME;

pub fn from_default_path() -> Config {
    let default_path = if let Some(path) = default_config_path() {
        path
    } else {
        println!("Erroneous default config path");
        process::exit(1)
    };
    from_path(&default_path)
}

pub fn from_path(path: &Path) -> Config {
    if let Some(config) = load_config(&path) {
        config
    } else {
        println!("Erroneous config path");
        process::exit(1)
    }
}

pub fn default_config_path() -> Option<PathBuf> {
    let project = ProjectDirs::from("rs", APP_NAME, APP_NAME)?;
    Some(
        [
            project.config_dir().to_str()?,
            &format!("{}.toml", APP_NAME),
        ]
        .iter()
        .collect(),
    )
}

fn load_config(path: &Path) -> Option<Config> {
    debug!("Trying to open config file under {}", path.to_str()?);
    let cfg_str = match fs::read_to_string(path) {
        Ok(cfg_str) => Some(cfg_str),
        Err(err) => match err.kind() {
            io::ErrorKind::NotFound => None,
            _ => malformed_config(err),
        },
    };

    if cfg_str.is_none() {
        warn!(
            "No config file found under {}, using defaults",
            path.to_str()?
        );
        return Some(Config::default());
    }
    match toml::from_str(&cfg_str?) {
        Ok(config) => Some(config),
        Err(err) => malformed_config(err),
    }
}

fn malformed_config<E: Display>(err: E) -> ! {
    error!("Error parsing config file: {}", err.to_string());
    println!("Malformed config file");
    process::exit(1)
}

#[test]
fn path_not_found() {
    let config = load_config(Path::new(""));
    assert!(config.is_some());
    assert_eq!(config.unwrap(), Config::default());
}
