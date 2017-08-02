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

/// This enum represents symbols a meal is annotated with.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[allow(missing_docs)]
pub enum Symbol {
    Pig,
    Cattle,
    Poultry,
    Fish,
    Game,
    Lamb,
    Vegan,
    Bio,
    Vegetarian,
    Alcohol,
    SoupOfTheDay,
    MensaVital,
    Garlic,
    AnimalWelfare,
}

impl std::str::FromStr for Symbol {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Symbol Schwein" => Ok(Symbol::Pig),
            "Symbol Rind" => Ok(Symbol::Cattle),
            "Symbol Geflügel" => Ok(Symbol::Poultry),
            "Symbol Fisch" => Ok(Symbol::Fish),
            "Symbol Wild" => Ok(Symbol::Game),
            "Symbol Lamm" => Ok(Symbol::Lamb),
            "Symbol vegan" => Ok(Symbol::Vegan),
            "Symbol Bio" => Ok(Symbol::Bio),
            "Symbol vegetarisch" => Ok(Symbol::Vegetarian),
            "Symbol enth�lt Alkohol" => Ok(Symbol::Alcohol),
            "Symbol Suppe" => Ok(Symbol::SoupOfTheDay),
            "Symbol MensaVital" => Ok(Symbol::MensaVital),
            "Symbol Knoblauch" => Ok(Symbol::Garlic),
            "Symbol artgerechte Tierhaltung" => Ok(Symbol::AnimalWelfare),
            _ => Err(Error::Creation("symbol", s.to_owned(), None)),
        }
    }
}
