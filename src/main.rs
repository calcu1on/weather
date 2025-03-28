use reqwest::header::USER_AGENT;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct WeatherOfficeLocation {
    x: i32,
    y: i32,
    code: String,
}

fn main() {
    let location = WeatherOfficeLocation {
        x: 75,
        y: 59,
        code: "GYX".to_string(),
    };
    let weather = fetch_weather(location);
    print!("{}", weather);
}

fn fetch_weather(location: WeatherOfficeLocation) -> String {
    let url = format!(
        "https://api.weather.gov/gridpoints/{}/{},{}/forecast/hourly",
        location.code,
        location.x,
        location.y
    );
    let client = reqwest::blocking::Client::new();
    let forecast = client.get(&url).header(USER_AGENT, "My Rust Program 1.0").send().expect("Unable to get data").text().unwrap().to_string();
    let json: serde_json::Value =
        serde_json::from_str(&forecast).expect("JSON was not well-formatted");
    let weather_periods = json.get("properties").expect("No properties found").get("periods").expect("No periods found.");

    let current_temp: String = weather_periods[0].get("temperature").expect("No temperature found.").to_string();
    let short_forecast: String = weather_periods[0].get("shortForecast").expect("No short forecast.").to_string();
    format!("Conditions are currently {}, it is {} degrees", short_forecast, current_temp)
}

