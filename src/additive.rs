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

use Error;
use std;

/// An `Additive` is used to represent additives of a meal.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[allow(missing_docs)]
pub enum Additive {
    FoodColoring,
    FoodPreservatives,
    AntiOxidants,
    FlavorEnhancer,
    Sulfurized,
    Waxed,
    Blackend,
    Phosphates,
    Sweetener,
    Phenylalanine,
}

/// This method is used to create an `Additive` from the additive strings
/// used on the canteen website.
///
/// # Examples
///
/// ```
/// use ovgu_canteen::Additive;
/// use std::str::FromStr;
///
/// let additive = Additive::from_str("(2)");
/// ```
impl std::str::FromStr for Additive {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "(1)" => Ok(Additive::FoodColoring),
            "(2)" => Ok(Additive::FoodPreservatives),
            "(3)" => Ok(Additive::AntiOxidants),
            "(4)" => Ok(Additive::FlavorEnhancer),
            "(5)" => Ok(Additive::Sulfurized),
            "(6)" => Ok(Additive::Waxed),
            "(7)" => Ok(Additive::Blackend),
            "(8)" => Ok(Additive::Phosphates),
            "(9)" => Ok(Additive::Sweetener),
            "(10)" => Ok(Additive::Phenylalanine),
            _ => Err(Error::Creation("additive", s.to_owned(), None)),
        }
    }
}
