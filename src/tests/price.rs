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

//! This module contains unit tests for `ovgu::canteen::Price`.

use ovgu::canteen::Price;
use std::str::FromStr;

#[test]
fn from_str()
{
    let price = Price { student: 1.40, staff: 2.50, guest: 3.60 };
    assert_eq!(Price::from_str("1,40 | 2,50 | 3,60").unwrap(), price);
}
