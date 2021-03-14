use std::{env, str};

use assert_cmd::Command;

use crate::common::create_cmd;
use crate::common::APP_NAME;
use crate::common::WEDDER_WEATHER_API_KEY;

mod common;

#[test]
fn debug() {
    create_cmd().arg("-d").assert().success();
}

#[test]
fn default_config_path() {
    let path = &create_cmd()
        .arg("-p")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let uft8 = str::from_utf8(path);
    assert!(uft8.is_ok());
    let filename = format!("{}.toml", APP_NAME);
    assert!(uft8.unwrap().contains(&filename));
}

#[test]
fn negative_interval() {
    Command::cargo_bin(APP_NAME)
        .unwrap()
        .arg("-k")
        .arg("mock")
        .arg("-w")
        .arg("OpenWeatherMap")
        .arg("-l")
        .arg("IpApi")
        .arg("-i")
        .arg("-1000")
        .assert()
        .success();
}

#[test]
fn negative_coordinates() {
    create_cmd()
        .arg("--lat")
        .arg("-53.154552")
        .arg("--lon")
        .arg("-70.897690")
        .assert()
        .success();
}

#[test]
fn no_arg() {
    create_cmd().arg("-c").assert().failure();
}

#[test]
fn wrong_arg() {
    create_cmd().arg("-w").arg("wrong").assert().failure();
}

#[test]
fn no_api_key() {
    env::remove_var(WEDDER_WEATHER_API_KEY);
    Command::cargo_bin(APP_NAME)
        .unwrap()
        .arg("-k")
        .arg("")
        .assert()
        .failure()
        .stdout("No API key\n");
}

#[test]
fn no_api_key_but_env() {
    env::set_var(WEDDER_WEATHER_API_KEY, "1234");
    Command::cargo_bin(APP_NAME)
        .unwrap()
        .arg("-i")
        .arg("-1")
        .arg("-k")
        .arg("")
        .assert()
        .success();
}
