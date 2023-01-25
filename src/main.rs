#![allow(dead_code)]
#[allow(unused)]


use chrono::NaiveDateTime;
use serde::Deserialize;

use clearscreen;
use serde_json;

mod input;

mod spinner;

#[derive(Deserialize)]
struct WeatherData {
    coord: Coord,
    weather: Vec<Weather>,
    base: String,
    main: Main,
    visibility: i32,
    wind: Wind,
    clouds: Clouds,
    dt: i32,
    sys: Sys,
    timezone: i32,
    id: i32,
    name: String,
    cod: i32,
}

#[derive(Deserialize)]
struct Weather {
    id: i32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Deserialize)]
struct Coord {
    lon: f32,
    lat: f32,
}

#[derive(Deserialize)]
struct Main {
    temp: f32,
    feels_like: f32,
    temp_min: f32,
    temp_max: f32,
    pressure: i32,
    humidity: i32,
}

#[derive(Deserialize)]
struct Wind {
    speed: f32,
    deg: i32,
}

#[derive(Deserialize)]
struct Clouds {
    all: i32,
}

#[derive(Deserialize)]
struct Sys {
    country: String,
    sunrise: i32,
    sunset: i32,
}

async fn get_weather(city: String) -> Result<WeatherData, Box<dyn std::error::Error>> {
    if city == "" {
        panic!("Please enter a city");
    }

    const API_KEY: &str = "a665098cb9d57501cf18750b913f5606";
    let client = reqwest::Client::new();
    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&appid={}", city, API_KEY);
    let response = client.get(&url).send().await?;
    if &response.status().to_string() == "404 Not Found" {
        panic!("You're sure '{city}' is a valid city?");
    }
    let body = response.text().await?;
    let new_weather = match serde_json::from_str::<WeatherData>(&body) {
        Ok(weather) => weather,
        Err(e) => {
            return Err(e.into());
        }
    };

    Ok(new_weather)
}

#[tokio::main]
async fn main() {
    spinner::spinner_loader();
    clearscreen::clear().expect("failed to clear screen");
    let welcome = format!(
        r"
    __          __        _   _                                      
    \ \        / /       | | | |                   /\                
     \ \  /\  / /__  __ _| |_| |__   ___ _ __     /  \   _ __  _ __  
      \ \/  \/ / _ \/ _` | __| '_ \ / _ \ '__|   / /\ \ | '_ \| '_ \ 
       \  /\  /  __/ (_| | |_| | | |  __/ |     / ____ \| |_) | |_) |
        \/  \/ \___|\__,_|\__|_| |_|\___|_|    /_/    \_\ .__/| .__/ 
                                                        | |   | |    
                                                        |_|   |_|    
    "
    );
    println!(
        "{welcome}\n
    Welcome to the weather CLI app\n
    Please enter city name or pincode.
    "
    );
    loop {

        let city = input::prompt();
        let w_data = get_weather(city).await.unwrap();
        let vector_data: Vec<String> = vec![
            w_data.name,
            w_data.sys.country,
        w_data.main.temp.to_string(),
        w_data.wind.speed.to_string(),
        w_data.main.temp_max.to_string(),
        w_data.main.temp_min.to_string(),
        w_data.main.feels_like.to_string(),
        w_data.visibility.to_string(),
        NaiveDateTime::from_timestamp_opt(w_data.sys.sunrise.into(), 0).expect("REASON").to_string(),
        NaiveDateTime::from_timestamp_opt(w_data.sys.sunset.into(), 0).expect("REASON").to_string(),
    ];
    let msg = format!(
        "
        =================================\n
        City: {0} || Country: {1}\n
        Temp: {2}ºF || Wind: {3} knot\n
        Max Temp: {4}ºF || Min Temp {5}ºF\n
        Feels like {6}ºF || Visibility: {7}m\n
        Sun rise: {8} || Sun set: {9}\n
        ===================================
        ",
        vector_data[0],
        vector_data[1],
        vector_data[2],
        vector_data[3],
        vector_data[4],
        vector_data[5],
        vector_data[6],
        vector_data[7],
        vector_data[8],
        vector_data[9],
    );
    println!("{msg}");
}
}
