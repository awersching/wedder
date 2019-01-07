#[derive(Serialize, Deserialize)]
pub struct Config {
    pub weather_api_key: String,
    pub city: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            weather_api_key: "key".to_string(),
            city: "city".to_string(),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        confy::load("currweather").unwrap()
    }
}
