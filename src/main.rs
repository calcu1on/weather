use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use serde_alias::serde_alias;
use tabled::{Tabled, Table};
use tabled::settings::{Style, Alignment, object::Columns};

#[derive(Debug, Serialize, Deserialize)]
struct ForecastWrapper {
    properties: Properties,
}

#[derive(Debug, Serialize, Deserialize)]
struct Properties {
    periods: Vec<WeatherPeriod>,
}

#[serde_alias(CamelCase,SnakeCase)]
#[derive(Debug, Serialize, Deserialize, Tabled)]
struct Precipitation {
    value: f32,
}

#[serde_alias(CamelCase,SnakeCase)]
#[derive(Serialize, Deserialize, Debug, Tabled)]
struct WeatherPeriod {
    number: i32,   
    start_time: String,
    end_time: String,
    temperature: u64,
    temperature_unit: char,
    wind_direction: String,
    wind_speed: String,
    short_forecast: String,
    // probability_of_precipitation: Precipitation,
}

#[derive(Serialize, Deserialize, Debug)]
struct WeatherOfficeLocation {
    x: i32,
    y: i32,
    code: String,
}

impl WeatherOfficeLocation {
    pub fn build_url(&self) -> String {
        format!(
            "https://api.weather.gov/gridpoints/{}/{},{}/forecast/hourly",
            self.code,
            self.x,
            self.y
        )
    }
}

fn main() {
    // Set the location for the API request.
    let location = WeatherOfficeLocation {
        x: 75,
        y: 59,
        code: "GYX".to_string(),
    };
    // Get entire forecast response.
    let full_forecast = get_full_forecast(location);
    // Get the current forecast.
    let current_forecast = get_forecast_from_period(&full_forecast, 0); 
    // Get 2 days of forecasts.
    let upcoming_forecast = &full_forecast[0..48];
    let mut table = Table::new(upcoming_forecast);
    table.with(Style::modern());
    table.modify(Columns::first(), Alignment::right());

    println!("--------------------------------------------------------------");
    println!("{}", current_forecast);
    println!("--------------------------------------------------------------");
    print!("{}", table);
}

// Gets the full forecast from the response.
fn get_full_forecast(location: WeatherOfficeLocation) -> Vec<WeatherPeriod> {
    let url = WeatherOfficeLocation::build_url(&location);
    let client = reqwest::blocking::Client::new();
    let forecast = client.get(&url).header(USER_AGENT, "My SuperAwesome Weather App").send().expect("Unable to get data").text().unwrap().to_string();
    let json: ForecastWrapper = serde_json::from_str(&forecast).expect("JSON was not well-formatted");
    let periods: Vec<WeatherPeriod> = json.properties.periods.into_iter().collect();

    periods
}

// allows retrieving a specific period from forecast.
fn get_forecast_from_period(periods: &Vec<WeatherPeriod>, target: usize) -> String {
    let current_temp: u64 = periods[target].temperature;
    let temp_units: char = periods[target].temperature_unit;
    let short_forecast: String = periods[target].short_forecast.to_owned();
    let current_weather = format!("Conditions are currently: {}, it is {}{}", short_forecast, current_temp, temp_units);

    current_weather
}
