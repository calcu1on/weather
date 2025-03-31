mod weather;
use tabled::Table;
use tabled::settings::{
    peaker::Priority, Width, Style, Alignment, object::Columns
};

fn main() {
    let location = weather::WeatherOfficeLocation {
        x: 75,
        y: 59,
        code: "GYX".to_string(),
    };
    let entire_forecast: Vec<weather::WeatherPeriod> = weather::get_full_forecast(location);
    let mut table = Table::new(entire_forecast);
    table.with(Style::modern());
    table.with((
        Width::wrap(210).priority(Priority::max(true)),
        Width::increase(50).priority(Priority::min(true)),
    ));
    table.modify(Columns::first(), Alignment::right());
    println!("{}", table);
}

