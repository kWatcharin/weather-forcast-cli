#![allow(unused)]
mod env;
mod models;
mod services;

use env::base_env::{load_env, API_KEY};
use models::openweathermap::WeatherResponse;
use services::{openweathermap::{get_weather_info, get_temperature_imoji}, Result};

use std::sync::Arc;
use std::process;
use std::io;
use colored::*;


fn main() -> Result<()> {
    load_env();
    let ref_api_key = Arc::clone(&API_KEY);
    let api_key = &*ref_api_key
        .read()
        .unwrap();


    println!("{}", "🌈 Welcome to weather station 🌞".bright_blue());
    

    loop {
        println!("{}", "Please, enter the name of the city!".bright_green());
        let mut city = String::new();
        io::stdin().read_line(&mut city)
            .expect("Failed to read input!");
        let city = city.trim();

        println!(
            "{}", 
            "Please, enter the country code (e.g.,TH for Thailand or JP for Japan):".bright_green()
        );
        let mut country_code = String::new();
        io::stdin()
            .read_line(&mut country_code)
            .expect("Failed to read input!");
        let country_code = country_code.trim();


        let result = get_weather_info(city, country_code, api_key)?;
        let temperature = result.main.temp;
        let temperature_imoji = get_temperature_imoji(temperature);
        let temperature_text = format!("Temperature is {:?} °C {}.", temperature, temperature_imoji);
        println!("{}", &temperature_text.bright_cyan());
    

        println!(
            "{}",
            "Do you want to search for weather in another city (y for yes / n for no):".bright_green()
        );
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input!");
        let input = input.trim();

        match input {
            "n" => {
                println!("{} 🙏", "Exit program!".bright_purple());
                process::exit(0);
            },
            _ => ()
        }
    }

    Ok(())
}