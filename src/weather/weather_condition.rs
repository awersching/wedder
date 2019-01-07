pub enum WeatherCondition {
    ClearSky,
    FewClouds,
    Clouds,
    ManyClouds,
    Rain,
    HeavyRain,
    Thunderstorm,
    Snow,
    Mist,
}

impl WeatherCondition {
    pub fn get_icon(&self) -> String {
        match self {
            // TODO: better icons
            WeatherCondition::ClearSky => "".to_string(),
            WeatherCondition::FewClouds => "".to_string(),
            WeatherCondition::Clouds => "".to_string(),
            WeatherCondition::ManyClouds => "".to_string(),
            WeatherCondition::Rain => "".to_string(),
            WeatherCondition::HeavyRain => "".to_string(),
            WeatherCondition::Thunderstorm => "".to_string(),
            WeatherCondition::Snow => "".to_string(),
            WeatherCondition::Mist => "".to_string(),
        }
    }
}
