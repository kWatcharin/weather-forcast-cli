mod env;
mod models;
mod services;

use env::base_env::{load_env, API_KEY};
use services::{openweathermap::{get_weather_info, display_weather_info}, Result};

use std::sync::Arc;
use std::process;
use std::io;
use colored::*;


fn enter_button() -> String {
    let mut input = String::new().to_lowercase();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input!");
    input
}

fn exit_app() {
    println!("{} 🙏", "Exit App!".bright_purple());
    process::exit(0);
}


fn main() -> Result<()> {
    /* Set ENV */
    load_env();
    let ref_api_key = Arc::clone(&API_KEY);
    let api_key = &*ref_api_key
        .read()
        .unwrap();


    /* Start App! */
    println!("{}", "🌈 Welcome to weather station 🌞".bright_blue());
    
    loop {
        /* Enter your city'name. */
        println!(
            "{}", 
            "Please, enter the name of the city!".bright_green()
        );

        let city_input = enter_button();
        let city = city_input.trim();


        /* Enter your country code. */
        println!(
            "{}", 
            "Please, enter the country code (e.g.,TH for Thailand or JP for Japan):".bright_green()
        );

        let country_code_input = enter_button();
        let country_code = country_code_input.trim();


        /* Fetch API and display weather forcasting detail. */
        let response = get_weather_info(city, country_code, api_key)?;
        display_weather_info(&response);
    

        /* Enter y or n key for continue or exit app. */
        println!(
            "{}",
            "Do you want to search for weather in another city (y for yes / n for no):".bright_green()
        );

        let input = enter_button();
        match input.trim() {
            "y" => continue,
            key @ _ => {
                loop {
                    if key != "n" {
                        /* Restart force to enter y or n key for continue or exit app. */
                        println!(
                            "{}", 
                            "Please, select only y for yes or n for no!".bright_red()
                        );

                        let input = enter_button();
                        match input.trim() {
                            "y" => break,
                            "n" => exit_app(),
                            _ => continue
                        }
                    } else {
                        exit_app();
                    }
                }
            }
        }
    }
}