use std::process::Command;

use assert_cmd::prelude::*;

const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");

#[test]
fn default() {
    create_cmd().assert()
        .success()
        .stdout(" 7°C\n");
}

#[test]
fn temperature_kelvin() {
    let format = "<temperature_kelvin>, <kelvin_max>, <kelvin_min>";
    let expected = "280.32, 281.15, 279.15".to_string();
    test_format(format, expected);
}

#[test]
fn temperature_celsius() {
    let format = "<temperature_celsius>, <celsius_max>, <celsius_min>";
    let expected = "7, 8, 6".to_string();
    test_format(format, expected);
}

#[test]
fn temperature_fahrenheit() {
    let format = "<temperature_fahrenheit>, <fahrenheit_max>, <fahrenheit_min>";
    let expected = "45, 46, 43".to_string();
    test_format(format, expected);
}

#[test]
fn other() {
    let format = "<pressure>, <humidity>, <wind_speed>, <cloud_percentage>";
    let expected = "1012, 81, 4.1, 90".to_string();
    test_format(format, expected);
}

fn create_cmd() -> Command {
    let mut cmd = Command::cargo_bin(PKG_NAME).unwrap();
    cmd.arg("-k").arg("mock")
        .arg("-w").arg("OwmMock");
    cmd
}

fn test_format(format: &str, expected: String) {
    let mut cmd = create_cmd();
    cmd.arg("-f").arg(format);
    cmd.assert()
        .success()
        .stdout(format!("{}\n", expected));
}
