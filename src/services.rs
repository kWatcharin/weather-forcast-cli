use crate::models::openweathermap::WeatherResponse;
use colored::*;

pub type Result<T, E = reqwest::Error> = core::result::Result<T, E>;


pub mod openweathermap {
    use super::*;


    pub fn display_weather_info(response: &WeatherResponse) {
        let lat = response.coord.lat;
        let lon = response.coord.lon;
        let description = &response.weather[0].description;
        let temperature = response.main.temp;
        let humidity = response.main.humidity;
        let pressure = response.main.pressure;
        let wind_speed = response.wind.speed;

        let weather_text = format!(
            r#"
Weather in {} (latitude: {}, longitude: {}): {} {}
> Temperature: {:.1}°C,
> Humidity: {:.1}%,
> Pressure: {:.1} hPa,
> Wind Speed: {:.} m/s
"#, 
            response.name, lat, lon, description, get_temperature_imoji(temperature), temperature,
            humidity, pressure,wind_speed
        );

        let weather_text_colored = match description.as_str() {
            "clear sky" => {
                weather_text.bright_yellow()
            },
            "few clouds" | "scattered clouds" | "broken clouds" => {
                weather_text.bright_blue()
            },
            "overcast clouds" | "mist" | "haze" | "smoke" | "dust" | "fog" | "squalls" => {
                weather_text.dimmed()
            },
            "heavy intensity rain" | "moderate rain" | "shower rain" | "light rain" | "rain" | "thunderstorm" | "snow" => {
                weather_text.bright_cyan()
            },
            _ => {
                weather_text.normal()
            }
        };

        println!("{}", weather_text_colored);
    }
    
    pub fn get_temperature_imoji(temperature: f64) -> &'static str {
        match temperature {
            temperature if temperature < 0.0 => "❄️",
            temperature if temperature >= 0.0 && temperature < 5.0 => "☃️",
            temperature if temperature >= 5.0 && temperature < 10.0 => "☁️",
            temperature if temperature >= 10.0 && temperature < 20.0 => "🌥️",
            temperature if temperature >= 20.0 && temperature < 30.0 => "⛅",
            temperature if temperature >= 30.0 && temperature < 35.0 => "🌤️",
            _ => "🔥"
        }
    }
    
    pub fn get_weather_info(
        city: &str, country_code: &str, api_key: &str
    ) -> Result<WeatherResponse> {
        let url = format!(
            "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
            city, country_code, api_key
        );
        let request = reqwest::blocking::get(&url)?;
        let reseponse_json = request
            .json::<WeatherResponse>()?;
    
        Ok(reseponse_json)
    }
}