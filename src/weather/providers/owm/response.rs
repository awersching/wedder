use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OwmResponse {
    pub weather: Vec<OwmWeather>,
    pub main: OwmMain,
}

#[derive(Debug, Deserialize)]
pub struct OwmWeather {
    pub id: i32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Deserialize)]
pub struct OwmMain {
    pub temp: f32,
}
