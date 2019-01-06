pub mod geoclue;

pub trait CurrentLocation {
    fn current_location() -> Location;
}

pub struct Location {
    pub lat: String,
    pub lon: String,
}
