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

use {Day, Error, FromElement, Update};
use futures::Stream;
use futures::future::Future;
use hyper;
use hyper_openssl;
use num_cpus;
use openssl;
use scraper;
use std;
use std::clone::Clone;
use tokio_core::reactor::Core;

/// A canteen holds all the meals on all available days.
#[derive(Serialize, Deserialize, Debug)]
pub struct Canteen {
    /// This identifies a canteen.
    pub description: CanteenDescription,

    /// All available days holding the meals for this canteen.
    pub days: Vec<Day>,
}

/// This enum identifies a canteen.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum CanteenDescription {
    /// The canteen located downstairs.
    Downstairs,

    /// The canteen located upstairs.
    Upstairs,
}

impl Canteen {
    /// This method creates a new canteen instance from a given description.
    ///
    /// # Arguments
    ///
    /// * `desc`  - The identifier identifying the canteen.
    ///
    /// # Examples
    ///
    /// Create a new canteen:
    ///
    /// ```
    /// use ovgu_canteen::{Canteen, CanteenDescription};
    ///
    /// let canteen = Canteen::new(CanteenDescription::Downstairs).unwrap();
    /// println!("{}", canteen.days[0].meals[0].name);
    /// ```
    pub fn new(desc: CanteenDescription) -> Result<Self, Error> {
        let days = Self::load(desc.clone())?;
        Ok(Canteen {
            description: desc,
            days: days,
        })
    }

    /// This method updates the canteen from the website.
    pub fn update(&mut self) -> Result<(), Error> {
        let days = Self::load(self.description.clone())?;

        for day in days.iter() {
            if match self.days.iter_mut().find(|d| *d == day) {
                Some(ref mut d) => {
                    d.update(day)?;
                    false
                }
                None => true,
            }
            {
                self.days.push(day.clone());
            }
        }

        Ok(())
    }

    fn load(desc: CanteenDescription) -> Result<Vec<Day>, Error> {
        let mut core = Core::new().map_err(|e| {
            Error::Creation("Core", "tokio-core".to_owned(), Some(Box::new(e)))
        })?;

        let connector = hyper_openssl::HttpsConnector::new(num_cpus::get(), &core.handle())
            .map_err(|e: openssl::error::ErrorStack| {
                Error::Creation("HttpsConnector", "connector".to_owned(), Some(Box::new(e)))
            })?;
        let client = hyper::Client::configure().connector(connector).build(
            &core.handle(),
        );

        let uri = match desc {
            CanteenDescription::Downstairs => ovgu_canteen_url![downstairs],
            CanteenDescription::Upstairs => ovgu_canteen_url![upstairs],
        }.parse()
            .map_err(|e: hyper::error::UriError| {
                Error::Creation("parse", "uri".to_owned(), Some(Box::new(e)))
            })?;

        let work = client.get(uri).and_then(|res| res.body().concat2());
        let chunks = core.run(work).map_err(|e| {
            Error::Creation("get", "body".to_owned(), Some(Box::new(e)))
        })?;
        let body = std::str::from_utf8(&chunks).map_err(|e| {
            Error::Creation("body", "chunks".to_owned(), Some(Box::new(e)))
        })?;

        scraper::Html::parse_document(&body)
            .select(&ovgu_canteen_selector![day])
            .map(|day_node| Day::from_element(&day_node))
            .collect::<Result<Vec<Day>, Error>>()
    }
}

impl PartialEq for Canteen {
    fn eq(&self, other: &Self) -> bool {
        self.description == other.description
    }
}