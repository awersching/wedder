use crate::weather::weather_condition::WeatherCondition;

pub struct WeatherStatus {
    weather_condition: WeatherCondition,
    temperature: i64,
    unit: String,
}

impl WeatherStatus {
    pub fn new(condition_icon: WeatherCondition, temperature: i64, unit: String) -> Self {
        WeatherStatus {
            weather_condition: condition_icon,
            temperature,
            unit,
        }
    }
}

impl ToString for WeatherStatus {
    fn to_string(&self) -> String {
        format!(
            "{} {} {}",
            self.weather_condition.get_icon(),
            self.temperature,
            self.unit
        )
    }
}
