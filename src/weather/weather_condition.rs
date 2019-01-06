pub enum WeatherCondition {
    ClearSky,
    FewClouds,
    ScatteredClouds,
    BrokenClouds,
    ShowerRain,
    Rain,
    Thunderstorm,
    Snow,
    Mist,
}

impl WeatherCondition {
    pub fn get_icon(&self) -> String {
        match self {
            // TODO: better icons
            WeatherCondition::ClearSky => "".into(),
            WeatherCondition::FewClouds => "".into(),
            WeatherCondition::ScatteredClouds => "".into(),
            WeatherCondition::BrokenClouds => "".into(),
            WeatherCondition::ShowerRain => "".into(),
            WeatherCondition::Rain => "".into(),
            WeatherCondition::Thunderstorm => "".into(),
            WeatherCondition::Snow => "".into(),
            WeatherCondition::Mist => "".into(),
        }
    }
}
