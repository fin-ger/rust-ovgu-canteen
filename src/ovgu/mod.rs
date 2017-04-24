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

#![warn(missing_docs)]
#![doc(test(attr(allow(unused_variables), deny(warnings))))]

//! This module can gather information provided by the webpages of the
//! Otto-von-Guericke University Magdeburg (ovgu). This module currently
//! provides access to the canteen menus and is able to serialize the
//! information to a json string.

#[macro_use]
pub mod canteen;

mod error;

pub use self::error::Error;
