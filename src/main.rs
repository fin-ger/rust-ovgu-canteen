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

use ovgu::canteen::{Canteen, CanteenDescription};

fn main()
{
    let canteens = vec![
        Canteen::new(CanteenDescription::Downstairs).unwrap(),
        Canteen::new(CanteenDescription::Upstairs).unwrap()
    ];

    serde_json::to_writer_pretty(&mut std::io::stdout(), &canteens).unwrap();
    println!();
}
