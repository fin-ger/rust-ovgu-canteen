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

//! The `ovgu::canteen` module is able to parse the ovgu canteen website
//! and provide the information as serializable and deserializable structs.

#[macro_use]
mod constants;

mod additive;
mod allergenic;
mod symbol;
mod price;
mod meal;
mod day;
mod canteen;
mod from_element;

pub use self::additive::Additive;
pub use self::allergenic::Allergenic;
pub use self::canteen::{Canteen, CanteenDescription};
pub use self::day::Day;
pub use self::from_element::FromElement;
pub use self::meal::Meal;
pub use self::price::Price;
pub use self::symbol::Symbol;
