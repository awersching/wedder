pub mod owm;

#[derive(Serialize, Deserialize)]
pub enum WeatherProvider {
    OpenWeatherMap
}
