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

#[derive(Serialize, Deserialize, Debug)]
pub struct Canteen
{
    pub description: CanteenDescription,
    pub days: Vec<ovgu::canteen::Day>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum CanteenDescription
{
    Downstairs,
    Upstairs,
}

impl Canteen
{
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
