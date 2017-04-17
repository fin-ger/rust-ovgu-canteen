
use hyper;
use hyper_native_tls;
use ovgu;
use ovgu::canteen::FromElement;
use scraper;

use std::io::Read;

#[derive(Serialize, Deserialize)]
pub struct Canteen
{
    pub description: CanteenDescription,
    pub days: Vec<ovgu::canteen::Day>,
}

#[derive(Serialize, Deserialize)]
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
