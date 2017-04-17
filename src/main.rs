#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate hyper_native_tls;
extern crate scraper;
extern crate chrono;
extern crate serde;
extern crate serde_json;

#[macro_use]
mod ovgu;

use std::io::Read;
use ovgu::canteen::FromElement;

fn main()
{
    let ssl = hyper_native_tls::NativeTlsClient::new().expect("Cannot create TLS client!");
    let connector = hyper::net::HttpsConnector::new(ssl);
    let client = hyper::Client::with_connector(connector);
    let mut body = String::new();
    let mut response = client.get(ovgu_canteen_url![unten])
        .send()
        .expect("OVGU website is unreachable!");
    response.read_to_string(&mut body).expect("OVGU website returned invalid HTML!");
    let document = scraper::Html::parse_document(&body);

    let days = document.select(&ovgu_canteen_selector![day])
        .map(|day_node| ovgu::canteen::Day::from_element(&day_node))
        .collect::<Result<Vec<ovgu::canteen::Day>, ovgu::Error>>()
        .unwrap();

    serde_json::to_writer_pretty(&mut std::io::stdout(), &days).unwrap();
    println!();
}
