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

use crate::{Error, FromElement, Meal, Update};
use serde::{Serialize, Deserialize};
use chrono;
use scraper;

/// A `Day` holds all the meals that are available at the given day.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Day {
    /// The date of thsi day.
    pub date: chrono::NaiveDate,

    /// The meal available on this day.
    pub meals: Vec<Meal>,

    /// The side dishes available on this day.
    pub side_dishes: Vec<String>,
}

impl FromElement for Day {
    type Err = Error;
    fn from_element(day_node: &scraper::ElementRef) -> Result<Self, Self::Err> {
        let date = day_node
            .select(&ovgu_canteen_selector![date])
            .next()
            .and_then(|node| node.text().next())
            .ok_or(Error::NotAvailable { member: "date", object: "day" })
            .and_then(|date_str| {
                chrono::NaiveDate::parse_from_str(&date_str[date_str.len() - 10..], "%d.%m.%Y")
                    .map_err(|e| Error::InvalidValue {
                        member: "date",
                        object: "day",
                        cause: Box::new(e),
                    })
            })?;

        // we create meals from a given html node
        // then we collect an Iter<Result<Meal, Err>> into a Result<Vec<Meal>, Err>
        //  -> collect checks if any of the results failed
        //  -> therefore we can use ? on the collected Vec
        let meals = day_node
            .select(&ovgu_canteen_selector![meal])
            .map(|meal_node| Meal::from_element(&meal_node))
            .collect::<Result<Vec<Meal>, Error>>()?;

        let side_dishes = day_node
            .select(&ovgu_canteen_selector![side_dishes])
            .next()
            .and_then(|node| node.text().next())
            .ok_or(Error::NotAvailable { member: "side_dishes", object: "day" })
            .map(|side_dishes_str| {
                side_dishes_str[10..]
                    .trim()
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect()
            })?;

        Ok(Day {
            date: date,
            meals: meals,
            side_dishes: side_dishes,
        })
    }
}

impl Update for Day {
    type Err = Error;
    fn update(&mut self, from: &Self) -> Result<(), Self::Err> {
        for meal in from.meals.iter() {
            if match self.meals.iter_mut().find(|m| *m == meal) {
                Some(ref mut m) => {
                    m.update(meal)?;
                    false
                }
                None => true,
            }
            {
                self.meals.push(meal.clone());
            }
        }

        for side_dish in from.side_dishes.iter() {
            if !self.side_dishes.contains(side_dish) {
                self.side_dishes.push(side_dish.clone());
            }
        }

        Ok(())
    }
}

impl PartialEq for Day {
    fn eq(&self, other: &Self) -> bool {
        self.date == other.date
    }
}
