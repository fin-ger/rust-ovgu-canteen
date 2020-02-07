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

use crate::{Day, Error, FromElement, Update};
use serde::{Serialize, Deserialize};
use futures::TryStreamExt;
use hyper;
use hyper_tls;
use scraper;
use std;
use std::clone::Clone;

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
    /// # #[tokio::main]
    /// # async fn main() {
    /// let canteen = Canteen::new(CanteenDescription::Downstairs).await.unwrap();
    /// println!("{}", canteen.days[0].meals[0].name);
    /// # }
    /// ```
    pub async fn new(desc: CanteenDescription) -> Result<Self, Error> {
        let days = Self::load(desc.clone()).await?;
        Ok(Canteen {
            description: desc,
            days: days,
        })
    }

    /// This method updates the canteen from the website.
    pub async fn update(&mut self) -> Result<(), Error> {
        let days = Self::load(self.description.clone()).await?;

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

    async fn load(desc: CanteenDescription) -> Result<Vec<Day>, Error> {
        let connector = hyper_tls::HttpsConnector::new();
        let client = hyper::Client::builder().build::<_, hyper::Body>(connector);

        let uri = match desc {
            CanteenDescription::Downstairs => ovgu_canteen_url![downstairs],
            CanteenDescription::Upstairs => ovgu_canteen_url![upstairs],
        }.parse()
            .map_err(|e| {
                Error::Creation("parse", "uri".to_owned(), Some(Box::new(e)))
            })?;

        let mut resp = client.get(uri).await.map_err(|e| {
            Error::Creation("resp", "parse".to_owned(), Some(Box::new(e)))
        })?;
        let chunks: Vec<_> = resp.body_mut().try_collect().await.map_err(|e| {
            Error::Creation("chunks", "resp".to_owned(), Some(Box::new(e)))
        })?;
        let bytes = chunks.concat();
        let body = std::str::from_utf8(&bytes).map_err(|e| {
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
