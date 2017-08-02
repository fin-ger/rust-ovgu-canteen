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

use ovgu_canteen::ovgu::canteen::Canteen;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("examples/canteens.json").unwrap();
    let mut serialization = String::new();
    file.read_to_string(&mut serialization).unwrap();

    let mut canteens: Vec<Canteen> = serde_json::from_str(&serialization).unwrap();
    for canteen in canteens.iter_mut() {
        canteen.update().unwrap();
    }

    serde_json::to_writer_pretty(&mut std::io::stdout(), &canteens).unwrap();
    println!();
}
