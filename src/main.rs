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

    let days = document.select(&ovgu_canteen_selector![day])
        .map(|day_node| {
            let mut date_str = day_node.select(&ovgu_canteen_selector![date])
                .next()
                .and_then(|node| node.text().next())
                .expect("No date found!");
            date_str = &date_str[date_str.len() - 10..];
            let date = chrono::NaiveDate::parse_from_str(date_str, "%d.%m.%Y")
                .expect("Cannot parse Date!");

            let meals = day_node.select(&ovgu_canteen_selector![meal])
                .map(|element| {
                    let notes = match element.select(&ovgu_canteen_selector![notes])
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

                    let name = element.select(&ovgu_canteen_selector![name])
                        .next()
                        .and_then(|node| node.text().next())
                        .expect("No meal name found!")
                        .trim();

                    let price = ovgu::canteen::Price::from_str(element.select(&ovgu_canteen_selector![price])
                            .next()
                            .and_then(|node| node.text().last())
                            .expect("No price found!")
                            .trim())
                        .unwrap();

                    let symbols = element.select(&ovgu_canteen_selector![symbols])
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

            let side_dishes_str = day_node.select(&ovgu_canteen_selector![side_dishes])
                .next()
                .and_then(|node| node.text().next())
                .expect("Cannot find side dishes!");
            let side_dishes = side_dishes_str[10..]
                .trim()
                .split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            ovgu::canteen::Day {
                date: date,
                meals: meals,
                side_dishes: side_dishes,
            }
        })
        .collect::<Vec<ovgu::canteen::Day>>();

    serde_json::to_writer_pretty(&mut std::io::stdout(), &days).unwrap();
    println!();
}
