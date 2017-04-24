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

use hyper;
use hyper_native_tls;
use ovgu;
use ovgu::canteen::FromElement;
use scraper;

use std::io::Read;

/// A canteen holds all the meals on all available days.
#[derive(Serialize, Deserialize, Debug)]
pub struct Canteen
{
    /// This identifies a canteen.
    pub description: CanteenDescription,

    /// All available days holding the meals for this canteen.
    pub days: Vec<ovgu::canteen::Day>,
}

/// This enum identifies a canteen.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum CanteenDescription
{
    /// The canteen located downstairs.
    Downstairs,

    /// The canteen located upstairs.
    Upstairs,
}

impl Canteen
{
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
    /// use ovgu_canteen::ovgu::canteen::{Canteen, CanteenDescription};
    ///
    /// let canteen = Canteen::new(CanteenDescription::Downstairs).unwrap();
    /// println!("{}", canteen.days[0].meals[0].name);
    /// ```
    pub fn new(desc: CanteenDescription) -> Result<Self, ovgu::Error>
    {
        let ssl = hyper_native_tls::NativeTlsClient::new()
            .map_err(|e| ovgu::Error::Creation(
                "NativeTlsClient", "hyper".to_owned(), Some(Box::new(e))
            ))?;

        let connector = hyper::net::HttpsConnector::new(ssl);
        let client = hyper::Client::with_connector(connector);
        let mut body = String::new();
        let mut response = client
            .get(
                match desc
                {
                    CanteenDescription::Downstairs => ovgu_canteen_url![downstairs],
                    CanteenDescription::Upstairs => ovgu_canteen_url![upstairs],
                }
            )
            .send()
            .map_err(|e| ovgu::Error::NotAvailable("website", "canteen", Some(Box::new(e))))?;

        response
            .read_to_string(&mut body)
            .map_err(|e| ovgu::Error::InvalidValue("HTML for", "website", Some(Box::new(e))))?;

        let days = scraper::Html::parse_document(&body)
            .select(&ovgu_canteen_selector![day])
            .map(|day_node| ovgu::canteen::Day::from_element(&day_node))
            .collect::<Result<Vec<ovgu::canteen::Day>, ovgu::Error>>()?;

        Ok(Canteen {
            description: desc,
            days: days,
        })
    }
}
