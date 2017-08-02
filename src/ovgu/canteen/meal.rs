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

use ovgu;
use ovgu::canteen::{Additive, Allergenic, Price, Symbol};
use ovgu::canteen::Update;
use scraper;
use std::str::FromStr;

/// A `Meal` holds the meals name, the price, several symbols, additives,
/// and allergenics.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Meal {
    /// The name of the meal.
    pub name: String,

    /// The price of the meal.
    pub price: Price,

    /// Symbols that the meal is annotated with.
    pub symbols: Vec<Symbol>,

    /// Additives of the meal.
    pub additives: Vec<Additive>,

    /// Allergenics contained in the meal.
    pub allergenics: Vec<Allergenic>,
}

impl ovgu::canteen::FromElement for Meal {
    type Err = ovgu::Error;
    fn from_element(meal_node: &scraper::ElementRef) -> Result<Self, Self::Err> {
        let notes = meal_node
            .select(&ovgu_canteen_selector![notes])
            .next()
            .and_then(|node| node.text().next())
            .unwrap_or("")
            .trim()
            .split_whitespace()
            .filter(|item| !item.is_empty());

        let mut rest = vec![];
        let additives = notes
            .filter_map(|item| match ovgu::canteen::Additive::from_str(item) {
                Ok(v) => Some(v),
                Err(..) => {
                    rest.push(item);
                    None
                }
            })
            .collect();

        let allergenics = rest.iter()
            .map(|item| Allergenic::from_str(item))
            .collect::<Result<Vec<Allergenic>, ovgu::Error>>()?;

        let name = meal_node
            .select(&ovgu_canteen_selector![name])
            .next()
            .and_then(|node| node.text().next())
            .ok_or(ovgu::Error::NotAvailable("name", "meal", None))
            .map(|n| n.trim())?;

        let price = meal_node
            .select(&ovgu_canteen_selector![price])
            .next()
            .and_then(|node| node.text().last())
            .ok_or(ovgu::Error::NotAvailable("price", "meal", None))
            .and_then(|p| Price::from_str(p.trim()))?;

        let symbols = meal_node
            .select(&ovgu_canteen_selector![symbols])
            .map(|img| {
                img.value()
                    .attr("title")
                    .ok_or(ovgu::Error::NotAvailable("title", "symbol img tag", None))
                    .and_then(|t| ovgu::canteen::Symbol::from_str(t.trim()))
            })
            .collect::<Result<Vec<Symbol>, ovgu::Error>>()?;

        Ok(Meal {
            name: name.to_owned(),
            price: price,
            symbols: symbols,
            additives: additives,
            allergenics: allergenics,
        })
    }
}

impl Update for Meal {
    type Err = ovgu::Error;
    fn update(&mut self, from: &Self) -> Result<(), Self::Err> {
        self.price.update(&from.price)?;

        for symbol in from.symbols.iter() {
            if !self.symbols.contains(symbol) {
                self.symbols.push(symbol.clone());
            }
        }

        for additive in from.additives.iter() {
            if !self.additives.contains(additive) {
                self.additives.push(additive.clone());
            }
        }

        for allergenic in from.allergenics.iter() {
            if !self.allergenics.contains(allergenic) {
                self.allergenics.push(allergenic.clone());
            }
        }

        Ok(())
    }
}

impl PartialEq for Meal {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
