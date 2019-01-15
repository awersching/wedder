use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Response {
    pub weather: Vec<Weather>,
    pub main: Main,
}

#[derive(Debug, Deserialize)]
pub struct Weather {
    pub id: i32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Deserialize)]
pub struct Main {
    pub temp: f32,
}
