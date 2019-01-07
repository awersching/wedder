use crate::weather::weather_condition::WeatherCondition;

pub struct Weather {
    weather_condition: WeatherCondition,
    kelvin: f32,
}

impl Weather {
    pub fn new(weather_condition: WeatherCondition, kelvin: f32) -> Self {
        Weather {
            weather_condition,
            kelvin,
        }
    }

    pub fn format(&self, format: &String) -> String {
        format!(
            "{} {}",
            self.weather_condition.get_icon(),
            self.temp_to_celsius(),
        )
    }

    fn temp_to_celsius(&self) -> i32 {
        (self.kelvin - 273.15).round() as i32
    }

    fn temp_to_fahrenheit(&self) -> i32 {
        // TODO
        self.kelvin as i32
    }
}
