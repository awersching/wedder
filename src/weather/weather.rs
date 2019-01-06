use crate::weather::weather_condition::WeatherCondition;

pub struct Weather {
    weather_condition: WeatherCondition,
    temperature: i32,
    unit: String,
}

impl Weather {
    pub fn new(weather_condition: WeatherCondition, temperature: i32, unit: String) -> Self {
        Weather {
            weather_condition,
            temperature,
            unit,
        }
    }
}

impl ToString for Weather {
    fn to_string(&self) -> String {
        format!(
            "{} {}{}",
            self.weather_condition.get_icon(),
            self.temperature,
            self.unit
        )
    }
}
