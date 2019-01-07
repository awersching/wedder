use std::process;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub weather_api_key: String,
    pub interval: i32,
    pub city: String,
    pub format: String,
}

impl Default for Config {
    fn default() -> Self {
        Config::no_config_file_found()
    }
}

impl Config {
    pub fn new() -> Self {
        match confy::load("wedder") {
            Ok(cfg) => cfg,
            Err(_) => Config::no_config_file_found()
        }
    }

    fn no_config_file_found() -> Self {
        println!("Please add a configuration file");
        process::exit(1)
    }
}
