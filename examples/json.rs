// libovgu-canteen - A canteen parser module for ovgu.
//
// Copyright (C) 2017
//     Fin Christensen <christensen.fin@gmail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

extern crate ovgu_canteen;
extern crate serde_json;

use ovgu_canteen::{Canteen, CanteenDescription};

#[tokio::main]
async fn main() {
    let canteens = vec![tokio::try_join!(
        Canteen::new(CanteenDescription::UniCampusLowerHall),
        Canteen::new(CanteenDescription::UniCampusUpperHall),
        Canteen::new(CanteenDescription::Kellercafe),
        Canteen::new(CanteenDescription::Herrenkrug),
        Canteen::new(CanteenDescription::Stendal),
        Canteen::new(CanteenDescription::Wernigerode),
        Canteen::new(CanteenDescription::DomCafeteHalberstadt),
    ).unwrap()];

    serde_json::to_writer_pretty(&mut std::io::stdout(), &canteens).unwrap();
    println!();
}
