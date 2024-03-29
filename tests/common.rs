use assert_cmd::Command;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
#[allow(dead_code)]
pub const WEDDER_WEATHER_API_KEY: &str = "WEDDER_WEATHER_API_KEY";

pub fn create_cmd() -> Command {
    let mut cmd = Command::cargo_bin(APP_NAME).unwrap();
    cmd.arg("-c")
        .arg("")
        .arg("-k")
        .arg("mock")
        .arg("-w")
        .arg("OpenWeatherMap")
        .arg("-l")
        .arg("IpApi")
        .arg("-i")
        .arg("-1");
    cmd
}
