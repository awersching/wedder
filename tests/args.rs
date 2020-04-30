use std::str;

use assert_cmd::Command;

use crate::common::APP_NAME;
use crate::common::create_cmd;

mod common;

#[test]
fn debug() {
    create_cmd().arg("-d")
        .assert()
        .success();
}

#[test]
fn current_city() {
    create_cmd().arg("-C")
        .assert()
        .success();
}

#[test]
fn default_config_path() {
    let path = &create_cmd().arg("-p")
        .assert()
        .success()
        .get_output()
        .stdout.clone();
    let uft8 = str::from_utf8(path);
    assert!(uft8.is_ok());
    let filename = format!("{}.toml", APP_NAME);
    assert!(uft8.unwrap().contains(&filename));
}

#[test]
fn no_arg() {
    create_cmd().arg("-c")
        .assert()
        .failure();
}

#[test]
fn wrong_arg() {
    create_cmd().arg("-w").arg("wrong")
        .assert()
        .failure();
}

#[test]
fn no_api_key() {
    Command::cargo_bin(APP_NAME).unwrap()
        .arg("-k").arg("")
        .assert()
        .failure()
        .stdout("No API key\n");
}
