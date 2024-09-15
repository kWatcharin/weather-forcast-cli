#![allow(unused)]
mod env;
mod models;
use env::base_env::{load_env, API_KEY};
use models::openweathermap::WeatherResponse;

use serde::Deserialize;
use std::sync::Arc;
use colored::*;

type Result<T, E = reqwest::Error> = core::result::Result<T, E>;



fn get_weather_info(
    city: &str, country_code: &str, api_key: &str
) -> Result<WeatherResponse> {
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );
    let request = reqwest::blocking::get(&url)?;
    let reseponse_json = request.json::<WeatherResponse>()?;

    Ok(reseponse_json)
}


fn display_weather_info(response: &WeatherResponse) {
    todo!()
}


fn get_temperature_imoji() -> &'static str {
    todo!()
}


fn main() -> Result<()> {
    load_env();
    let ref_api_key = Arc::clone(&API_KEY);
    let api_key = &*ref_api_key
        .read()
        .unwrap();

    println!("{}", "🌈 Welcome to weather status 🌞".bright_yellow());
    
    let result = get_weather_info("Bangkok", "TH", api_key)?;
    println!("{:?}", result);
    Ok(())
}
