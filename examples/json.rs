extern crate ovgu_canteen;
extern crate serde_json;

use ovgu_canteen::ovgu::canteen::{Canteen, CanteenDescription};

fn main()
{
    let canteens = vec![Canteen::new(CanteenDescription::Downstairs).unwrap(),
                        Canteen::new(CanteenDescription::Upstairs).unwrap()];

    serde_json::to_writer_pretty(&mut std::io::stdout(), &canteens).unwrap();
    println!();
}
