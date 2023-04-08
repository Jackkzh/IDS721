use reqwest::blocking::Client;
use scraper::{Html, Selector};

fn main() {
    // create a HTTP client
    let client = Client::new();

    // send a GET request to the website
    let url = "https://skiptheline.ncdot.gov/Webapp/Appointment/Index/a7ade79b-996d-4971-8766-97feb75254de";
    let response = client.get(url).send().unwrap();

    // parse the response HTML
    let body = response.text().unwrap();
    let document = Html::parse_document(&body);

    // find the available appointment times
    let selector = Selector::parse(".calendar-day.has-availability").unwrap();
    let mut appointments = Vec::new();
    for element in document.select(&selector) {
        let date_selector = Selector::parse(".calendar-day-header").unwrap();
        let date_element = element.select(&date_selector).next().unwrap();
        let date = date_element.inner_html().trim().to_string();

        let time_selector = Selector::parse(".calendar-day-time > a").unwrap();
        for time_element in element.select(&time_selector) {
            let time = time_element.inner_html().trim().to_string();
            appointments.push(format!("{} {}", date, time));
        }
    }

    // print the available appointment times
    if appointments.is_empty() {
        println!("No available appointment times found.");
    } else {
        println!("The next available appointment times are:");
        for appointment in appointments {
            println!("{}", appointment);
        }
    }
}
