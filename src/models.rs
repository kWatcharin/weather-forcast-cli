use serde::Deserialize;


pub mod openweathermap {
    use super::*;


    #[derive(Deserialize, Debug)]
    pub struct Main {
        pub temp: f64,
        pub feels_like: f64,
        pub humidity: f64,
        pub pressure: f64
    }

    #[derive(Deserialize, Debug)]
    pub struct Wind {
        pub speed: f64
    }

    #[derive(Deserialize, Debug)]
    pub struct Weather {
        pub id: i64,
        pub main: String,
        pub description: String,
        pub icon: String
    }

    #[derive(Deserialize, Debug)]
    pub struct Coord {
        pub lon: f64,
        pub lat: f64
    }

    #[derive(Deserialize, Debug)]
    pub struct WeatherResponse {
        pub coord: Coord,
        pub weather: Vec<Weather>,
        pub base: String,
        pub main: Main,
        pub wind: Wind,
        pub name: String
    }
}