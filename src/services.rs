use crate::models::openweathermap::WeatherResponse;

pub type Result<T, E = reqwest::Error> = core::result::Result<T, E>;


pub mod openweathermap {
    use super::*;


    pub fn display_weather_info(response: &WeatherResponse) {
        todo!()
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