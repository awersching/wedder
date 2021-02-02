use crate::common::create_cmd;

mod common;

#[test]
fn default() {
    create_cmd().assert().success().stdout(" 7°C\n");
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
    <temperature_max>, <temperature_min>";
    create_cmd()
        .arg("-f")
        .arg(format)
        .assert()
        .success()
        .stdout("7, N/A, 8, 6\n");
}

#[test]
fn temperature_fahrenheit() {
    let format = "<temperature>, <temperature_feels_like>, \
    <temperature_max>, <temperature_min>";
    create_cmd()
        .arg("-f")
        .arg(format)
        .arg("-t")
        .arg("Fahrenheit")
        .assert()
        .success()
        .stdout("45, N/A, 46, 43\n");
}

#[test]
fn temperature_kelvin() {
    let format = "<temperature>, <temperature_feels_like>, \
    <temperature_max>, <temperature_min>";
    create_cmd()
        .arg("-f")
        .arg(format)
        .arg("-t")
        .arg("Kelvin")
        .assert()
        .success()
        .stdout("280, N/A, 281, 279\n");
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
        .stdout("4\n");
}

#[test]
fn wind_speed_kmh() {
    let format = "<wind_speed>";
    create_cmd()
        .arg("-f")
        .arg(format)
        .assert()
        .success()
        .stdout("15\n");
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
        .stdout("9\n");
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
        .stdout("10000\n");
}

#[test]
fn distance_kilometer() {
    let format = "<visibility>";
    create_cmd()
        .arg("-f")
        .arg(format)
        .assert()
        .success()
        .stdout("10\n");
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
        .stdout("6\n");
}

#[test]
fn precipitation_millimeter() {
    let format = "<precipitation>";
    create_cmd()
        .arg("-f")
        .arg(format)
        .assert()
        .success()
        .stdout("N/A\n");
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
        .stdout("N/A\n");
}

#[test]
fn other() {
    let format = "<pressure>, <humidity>, <cloud_percentage>";
    create_cmd()
        .arg("-f")
        .arg(format)
        .assert()
        .success()
        .stdout("1012, 81, 90\n");
}
