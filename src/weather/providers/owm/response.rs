use serde::Deserialize;

#[derive(Deserialize)]
pub struct Response {
    pub weather: Vec<Weather>,
    pub main: Main,
}

#[derive(Deserialize)]
pub struct Weather {
    pub id: i32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Deserialize)]
pub struct Main {
    pub temp: f32,
}
