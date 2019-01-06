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
            ClearSky => String::from(""),
            FewClouds => String::from(""),
            ScatteredClouds => String::from(""),
            BrokenClouds => String::from(""),
            ShowerRain => String::from(""),
            Rain => String::from(""),
            Thunderstorm => String::from(""),
            Snow => String::from(""),
            Mist => String::from(""),
        }
    }
}
