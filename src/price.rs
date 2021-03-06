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

use crate::{Error, Update};
use serde::{Serialize, Deserialize};
use std;

/// This struct represents the price of a meal.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Price {
    /// The price for students.
    pub student: f32,

    /// The price for staff.
    pub staff: f32,

    /// The price for guests.
    pub guest: f32,
}

impl std::str::FromStr for Price {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let replaced = str::replace(s, ",", ".");
        let mut split = replaced.split(" | ");

        let price_student = split
            .next()
            .ok_or(Error::NotAvailable { object: "price", member: "student" })
            .and_then(|num| {
                num.parse::<f32>().map_err(|e| Error::InvalidValue {
                    object: "price",
                    member: "student",
                    cause: Box::new(e),
                })
            })?;

        let price_staff = split
            .next()
            .ok_or(Error::NotAvailable { object: "price", member: "staff" })
            .and_then(|num| {
                num.parse::<f32>().map_err(|e| Error::InvalidValue {
                    object: "price",
                    member: "staff",
                    cause: Box::new(e),
                })
            })?;

        let price_guest = split
            .next()
            .ok_or(Error::NotAvailable { object: "price", member: "guest" })
            .and_then(|num| {
                num.parse::<f32>().map_err(|e| Error::InvalidValue {
                    object: "price",
                    member: "guest",
                    cause: Box::new(e),
                })
            })?;

        Ok(Price {
            student: price_student,
            staff: price_staff,
            guest: price_guest,
        })
    }
}

impl Update for Price {
    type Err = Error;
    fn update(&mut self, from: &Self) -> Result<(), Self::Err> {
        self.student = from.student;
        self.staff = from.staff;
        self.guest = from.guest;

        Ok(())
    }
}
