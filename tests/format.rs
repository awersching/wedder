use crate::common::create_cmd;

mod common;

#[test]
fn default() {
    create_cmd().assert().success().stdout(" 2°C\n");
}

#[test]
fn city() {
    let format = "<city>";
    create_cmd()
        .arg("-f")
        .arg(format)
        .assert()
        .success()
        .stdout("Montreal\n");
}

#[test]
fn temperature_celsius() {
    let format = "<temperature>, <temperature_feels_like>, \
    <temperature_max>, <temperature_min>, <dew_point>";
    create_cmd()
        .arg("-f")
        .arg(format)
        .assert()
        .success()
        .stdout("2, -3, 6, 0, 1\n");
}

#[test]
fn temperature_fahrenheit() {
    let format = "<temperature>, <temperature_feels_like>, \
    <temperature_max>, <temperature_min>, <dew_point>";
    create_cmd()
        .arg("-f")
        .arg(format)
        .arg("-t")
        .arg("Fahrenheit")
        .assert()
        .success()
        .stdout("35, 27, 43, 32, 34\n");
}

#[test]
fn temperature_kelvin() {
    let format = "<temperature>, <temperature_feels_like>, \
    <temperature_max>, <temperature_min>, <dew_point>";
    create_cmd()
        .arg("-f")
        .arg(format)
        .arg("-t")
        .arg("Kelvin")
        .assert()
        .success()
        .stdout("275, 270, 279, 273, 274\n");
}

#[test]
fn wind_speed_ms() {
    let format = "<wind_speed>";
    create_cmd()
        .arg("-f")
        .arg(format)
        .arg("-s")
        .arg("Ms")
        .assert()
        .success()
        .stdout("3.6\n");
}

#[test]
fn wind_speed_kmh() {
    let format = "<wind_speed>";
    create_cmd()
        .arg("-f")
        .arg(format)
        .assert()
        .success()
        .stdout("13.0\n");
}

#[test]
fn wind_speed_mph() {
    let format = "<wind_speed>";
    create_cmd()
        .arg("-f")
        .arg(format)
        .arg("-s")
        .arg("Mph")
        .assert()
        .success()
        .stdout("8.1\n");
}

#[test]
fn distance_meter() {
    let format = "<visibility>";
    create_cmd()
        .arg("-f")
        .arg(format)
        .arg("-D")
        .arg("Meter")
        .assert()
        .success()
        .stdout("6437.0\n");
}

#[test]
fn distance_kilometer() {
    let format = "<visibility>";
    create_cmd()
        .arg("-f")
        .arg(format)
        .assert()
        .success()
        .stdout("6.4\n");
}

#[test]
fn distance_mile() {
    let format = "<visibility>";
    create_cmd()
        .arg("-f")
        .arg(format)
        .arg("-D")
        .arg("Mile")
        .assert()
        .success()
        .stdout("4.0\n");
}

#[test]
fn precipitation_millimeter() {
    let format = "<precipitation>";
    create_cmd()
        .arg("-f")
        .arg(format)
        .assert()
        .success()
        .stdout("0.074\n");
}

#[test]
fn precipitation_inch() {
    let format = "<precipitation>";
    create_cmd()
        .arg("-f")
        .arg(format)
        .arg("-P")
        .arg("Inch")
        .assert()
        .success()
        .stdout("0.003\n");
}

#[test]
fn other() {
    let format = "<pressure>, <humidity>, <clouds>, \
    <precipitation_chance>, <uv_index>, <air_quality_index>";
    create_cmd()
        .arg("-f")
        .arg(format)
        .assert()
        .success()
        .stdout("1017, 97, 90, 20, 0, 1\n");
}
