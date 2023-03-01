use chrono::{NaiveDate, Duration};
use reqwest;

#[derive(Debug, serde::Deserialize)]
struct Holiday {
    name: String,
    date: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read input date from the user
    let input_date = read_input_date()?;

    // Get holidays from the API
    let holidays = get_holidays()?;

    // Calculate the differences between the input date and holiday dates
    let mut diffs: Vec<_> = holidays
        .iter()
        .map(|holiday| {
            let holiday_date = NaiveDate::parse_from_str(&holiday.date, "%Y-%m-%d").unwrap();
            //let diff = (holiday_date - input_date).abs();
            let diff = holiday_date.signed_duration_since(input_date).num_days();

            (holiday, diff)
        })
        .collect();

    // Sort the holidays by their differences to the input date
    diffs.sort_by_key(|&(_, diff)| diff);

    // Print the 5 nearest holidays
    for &(holiday, _) in diffs.iter().take(5) {
        println!("{}: {}", holiday.date, holiday.name);
    }

    Ok(())
}

fn read_input_date() -> Result<NaiveDate, Box<dyn std::error::Error>> {
    loop {
        println!("Enter a date (YYYY-MM-DD):");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();
        if let Ok(date) = NaiveDate::parse_from_str(input, "%Y-%m-%d") {
            return Ok(date);
        } else {
            println!("Invalid date format. Try again.");
        }
    }
}

use serde_json::Value;


fn get_holidays() -> Result<Vec<Holiday>, Box<dyn std::error::Error>> {
    let url = "https://date.nager.at/api/v3/NextPublicHolidays/US";
    //let resp = reqwest::blocking::get(url)?.json::<Vec<Holiday>>()?;
    let resp = reqwest::blocking::get(url)?.text()?;
    let json: Vec<Holiday> = serde_json::from_str(&resp)?;
    Ok(json)
}