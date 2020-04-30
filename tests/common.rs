use assert_cmd::Command;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

pub fn create_cmd() -> Command {
    let mut cmd = Command::cargo_bin(APP_NAME).unwrap();
    cmd.arg("-k").arg("mock")
        .arg("-w").arg("OwmMock")
        // invalid path -> config defaults -> interval is None
        .arg("-c").arg("");
    cmd
}
