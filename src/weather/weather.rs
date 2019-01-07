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
        let mut formatted = format.clone();
        formatted = formatted.replace(
            "<icon>",
            &*self.weather_condition.get_icon(),
        );

        if formatted.contains("<temperature_fahrenheit>") {
            formatted = formatted.replace(
                "<temperature_fahrenheit>",
                &*self.temp_to_fahrenheit().to_string(),
            );
        } else if formatted.contains("<temperature_kelvin>") {
            formatted = formatted.replace(
                "<temperature_kelvin>",
                &*(self.kelvin.round() as i32).to_string(),
            );
        } else {
            formatted = formatted.replace(
                "<temperature_celsius>",
                &*self.temp_to_celsius().to_string(),
            );
        }
        formatted
    }

    fn temp_to_celsius(&self) -> i32 {
        (self.kelvin - 273.15).round() as i32
    }

    fn temp_to_fahrenheit(&self) -> i32 {
        // TODO
        self.kelvin as i32
    }
}
