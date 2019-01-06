use config::Config;
use config::File;

pub struct Settings {
    pub city: String,
    pub weather_api_key: String,
}

impl Settings {
    pub fn new() -> Self {
        let mut config = Config::default();
        config.merge(File::with_name("settings.toml"))
            .unwrap();

        Settings {
            city: config.get_str("city").unwrap(),
            weather_api_key: config.get_str("weather_api_key").unwrap(),
        }
    }
}
