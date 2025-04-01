mod weather;
mod redsox;
use tabled::Table;
use tabled::settings::{
    peaker::Priority, Width, Style, Alignment, object::Columns
};

struct TableRow {
    date: String,
    name: String,
    temperature: u64,
    red_sox: String,
    detailed_forecast: String,
}

fn main() {
    // Set the weather location here.
    let location = weather::WeatherOfficeLocation {
        x: 75,
        y: 59,
        code: "GYX".to_string(),
    };
    // @todo - add a way to configure which teams to add
    // redsox, patriots
    // @todo - add a way to get news articles?
    // which sources do we want news from?
    // turn this from a weather app to "morning-coffee" or "daily download" 
    // or something like this....maybe use AI to come up with a name.
    let entire_forecast: Vec<weather::WeatherPeriod> = weather::get_full_forecast(location);
    // let sox_games: Vec<redsox::Game> = redsox::get_upcoming_games();
    for i in 0..entire_forecast.len() {
        let date = &entire_forecast[i].start_time[0..10];
        println!("{}", date);
    }
    // iterate over the entire forecast and move into table rows
    // panic!("exit");
    let mut table = Table::new(entire_forecast);
    table.with(Style::modern());
    table.with((
        Width::wrap(210).priority(Priority::max(true)),
        Width::increase(50).priority(Priority::min(true)),
    ));
    table.modify(Columns::first(), Alignment::right());
    println!("{}", table);
}

