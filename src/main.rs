#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate hyper_native_tls;
extern crate scraper;
extern crate chrono;
extern crate serde;
extern crate serde_json;

use std::io::Read;
use std::str::FromStr;

#[macro_use]
mod ovgu;

fn main()
{
    let ssl = hyper_native_tls::NativeTlsClient::new().expect("Cannot create TLS client!");
    let connector = hyper::net::HttpsConnector::new(ssl);
    let client = hyper::Client::with_connector(connector);
    let mut body = String::new();
    let mut response = client.get(ovgu_canteen_url![unten])
        .send()
        .expect("OVGU website is unreachable!");
    response.read_to_string(&mut body).expect("OVGU website returned invalid HTML!");
    let document = scraper::Html::parse_document(&body);

    let sel_day = scraper::Selector::parse(".mensa > table").unwrap();
    let sel_date = scraper::Selector::parse("thead > tr > td").unwrap();
    let sel_side_dishes = scraper::Selector::parse("tbody > tr:last-of-type > td").unwrap();
    let sel_meal = scraper::Selector::parse("tbody > tr:not(:last-of-type)").unwrap();
    let sel_name = scraper::Selector::parse("td:nth-of-type(1) > strong").unwrap();
    let sel_price = scraper::Selector::parse("td:nth-of-type(1)").unwrap();
    let sel_symbols = scraper::Selector::parse("td:nth-of-type(2) > div > img").unwrap();
    let sel_notes = scraper::Selector::parse("td:nth-of-type(2) > div:nth-of-type(2)").unwrap();

    let days = document.select(&sel_day)
        .map(|day_node| {
            let mut date_str = day_node.select(&sel_date)
                .next()
                .and_then(|node| node.text().next())
                .expect("No date found!");
            date_str = &date_str[date_str.len() - 10..];
            let date = chrono::NaiveDate::parse_from_str(date_str, "%d.%m.%Y")
                .expect("Cannot parse Date!");

            let meals = day_node.select(&sel_meal)
                .map(|element| {
                    let notes = match element.select(&sel_notes)
                            .next()
                            .and_then(|node| node.text().next())
                        {
                            Some(v) => v.trim(),
                            None => "",
                        }
                        .split_whitespace()
                        .take_while(|item| !item.is_empty());

                    let mut rest = vec![];
                    let additives = notes.filter_map(|item| {
                            match ovgu::canteen::Additive::from_str(item)
                            {
                                Ok(v) => Some(v),
                                Err(..) =>
                                {
                                    rest.push(item);
                                    None
                                }
                            }
                        })
                        .collect();
                    let allergenics = rest.iter()
                        .map(|item| {
                            ovgu::canteen::Allergenic::from_str(item)
                                .expect(format!("Cannot parse note '{}'!", item).as_str())
                        })
                        .collect();

                    let name = element.select(&sel_name)
                        .next()
                        .and_then(|node| node.text().next())
                        .expect("No meal name found!")
                        .trim();

                    let price = ovgu::canteen::Price::from_str(element.select(&sel_price)
                            .next()
                            .and_then(|node| node.text().last())
                            .expect("No price found!")
                            .trim())
                        .unwrap();

                    let symbols = element.select(&sel_symbols)
                        .map(|img| {
                            ovgu::canteen::Symbol::from_str(img.value()
                                    .attr("title")
                                    .expect("Symbol tag has no title!")
                                    .trim())
                                .unwrap()
                        })
                        .collect();

                    ovgu::canteen::Meal {
                        name: name.to_string(),
                        price: price,
                        symbols: symbols,
                        additives: additives,
                        allergenics: allergenics,
                    }
                })
                .collect();

            let side_dishes_str = day_node.select(&sel_side_dishes)
                .next()
                .and_then(|node| node.text().next())
                .expect("Cannot find side dishes!");
            let side_dishes = side_dishes_str[10..]
                .trim()
                .split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            ovgu::canteen::Day {
                date: ovgu::canteen::NaiveDateSerde(date),
                meals: meals,
                side_dishes: side_dishes,
            }
        })
        .collect::<Vec<ovgu::canteen::Day>>();

    serde_json::to_writer_pretty(&mut std::io::stdout(), &days).unwrap();
    println!();
}
