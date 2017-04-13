#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate hyper_native_tls;
extern crate scraper;
extern crate html5ever;
extern crate chrono;
extern crate serde;
extern crate serde_json;

use std::io::Read;
use std::str::FromStr;

macro_rules! ovgu_protocol {
    () => ( "https://" )
}

macro_rules! ovgu_mensa {
    () => ( concat!(ovgu_protocol!(), "www.studentenwerk-magdeburg.de/mensen-cafeterien/") )
}

macro_rules! ovgu_mensa_unten {
    () => ( concat!(ovgu_mensa!(), "mensa-unicampus/speiseplan-unten/") )
}

#[derive(Debug)]
enum Symbol {
    Pig,
    Cattle,
    Poultry,
    Fish,
    Game,
    Lamb,
    Vegan,
    Bio,
    Vegetarian,
    Alcohol,
    SoupOfTheDay,
    MensaVital,
    Garlic,
    AnimalWelfare,
}

impl FromStr for Symbol {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "Symbol Schwein" => Ok(Symbol::Pig),
            "Symbol Rind" => Ok(Symbol::Cattle),
            "Symbol Geflügel" => Ok(Symbol::Poultry),
            "Symbol Fisch" => Ok(Symbol::Fish),
            "Symbol Wild" => Ok(Symbol::Game),
            "Symbol Lamm" => Ok(Symbol::Lamb),
            "Symbol vegan" => Ok(Symbol::Vegan),
            "Symbol Bio" => Ok(Symbol::Bio),
            "Symbol vegetarisch" => Ok(Symbol::Vegetarian),
            "Symbol enth�lt Alkohol" => Ok(Symbol::Alcohol),
            "Symbol Suppe" => Ok(Symbol::SoupOfTheDay),
            "Symbol MensaVital" => Ok(Symbol::MensaVital),
            "Symbol Knoblauch" => Ok(Symbol::Garlic),
            "Symbol artgerechte Tierhaltung" => Ok(Symbol::AnimalWelfare),
            _ => Err(format!("Cannot create symbol from '{}'!", s)),
        }
    }
}

#[derive(Debug)]
enum Additive {
    FoodColoring,
    FoodPreservatives,
    AntiOxidants,
    FlavorEnhancer,
    Sulfurized,
    Waxed,
    Blackend,
    Phosphates,
    Sweetener,
    Phenylalanine,
}

impl FromStr for Additive {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "(1)" => Ok(Additive::FoodColoring),
            "(2)" => Ok(Additive::FoodPreservatives),
            "(3)" => Ok(Additive::AntiOxidants),
            "(4)" => Ok(Additive::FlavorEnhancer),
            "(5)" => Ok(Additive::Sulfurized),
            "(6)" => Ok(Additive::Waxed),
            "(7)" => Ok(Additive::Blackend),
            "(8)" => Ok(Additive::Phosphates),
            "(9)" => Ok(Additive::Sweetener),
            "(10)" => Ok(Additive::Phenylalanine),
            _ => Err(format!("Cannot create additive from '{}'!", s)),
        }
    }
}

#[derive(Debug)]
enum Allergenic {
    Wheat,
    Rye,
    Barley,
    Oat,
    Spelt,
    Kamut,
    Crustacean,
    Egg,
    Fish,
    Peanut,
    Soya,
    Lactose,
    Almond,
    Hazelnut,
    Walnut,
    Cashew,
    PecanNut,
    BrazilNut,
    Pistachio,
    MacadamiaNut,
    QueenslandNut,
    Celery,
    Mustard,
    Sesame,
    Sulphite,
    Lupin,
    Mollusc,
}

impl FromStr for Allergenic {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "(a1)" => Ok(Allergenic::Wheat),
            "(a2)" => Ok(Allergenic::Rye),
            "(a3)" => Ok(Allergenic::Barley),
            "(a4)" => Ok(Allergenic::Oat),
            "(a5)" => Ok(Allergenic::Spelt),
            "(a6)" => Ok(Allergenic::Kamut),
            "(b)" => Ok(Allergenic::Crustacean),
            "(c)" => Ok(Allergenic::Egg),
            "(d)" => Ok(Allergenic::Fish),
            "(e)" => Ok(Allergenic::Peanut),
            "(f)" => Ok(Allergenic::Soya),
            "(g)" => Ok(Allergenic::Lactose),
            "(h1)" => Ok(Allergenic::Almond),
            "(h2)" => Ok(Allergenic::Hazelnut),
            "(h3)" => Ok(Allergenic::Walnut),
            "(h4)" => Ok(Allergenic::Cashew),
            "(h5)" => Ok(Allergenic::PecanNut),
            "(h6)" => Ok(Allergenic::BrazilNut),
            "(h7)" => Ok(Allergenic::Pistachio),
            "(h8)" => Ok(Allergenic::MacadamiaNut),
            "(h9)" => Ok(Allergenic::QueenslandNut),
            "(i)" => Ok(Allergenic::Celery),
            "(j)" => Ok(Allergenic::Mustard),
            "(k)" => Ok(Allergenic::Sesame),
            "(l)" => Ok(Allergenic::Sulphite),
            "(m)" => Ok(Allergenic::Lupin),
            "(n)" => Ok(Allergenic::Mollusc),
            _ => Err(format!("Cannot create allergenic from '{}'!", s)),
        }
    }
}

#[derive(Debug)]
struct Price {
    student: f32,
    staff: f32,
    guest: f32,
}

fn f32_from_split<'a>(context: &'a str, split: Option<&'a str>) -> Result<f32, String> {
    match split {
        Some(num) => {
            match num.parse() {
                Ok(v) => Ok(v),
                Err(..) => Err(format!("Failed to parse {} '{}'!", context, num)),
            }
        }
        None => Err(format!("Cannot find {}!", context)),
    }
}

impl FromStr for Price {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        let replaced = str::replace(s, ",", ".");
        let mut split = replaced.split(" | ");
        let (student, staff, guest): (f32, f32, f32);
        student = match f32_from_split("student price", split.next()) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        staff = match f32_from_split("staff price", split.next()) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        guest = match f32_from_split("guest price", split.next()) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        Ok(Price {
            student: student,
            staff: staff,
            guest: guest,
        })
    }
}

#[derive(Debug)]
struct Meal<'a> {
    name: &'a str,
    price: Price,
    symbols: Vec<Symbol>,
    additives: Vec<Additive>,
    allergenics: Vec<Allergenic>,
}

#[derive(Debug)]
struct Day<'a> {
    date: chrono::NaiveDate,
    meals: Vec<Meal<'a>>,
    side_dishes: Vec<&'a str>,
}

fn main() {
    let ssl = hyper_native_tls::NativeTlsClient::new().expect("Cannot create TLS client!");
    let connector = hyper::net::HttpsConnector::new(ssl);
    let client = hyper::Client::with_connector(connector);
    let mut body = String::new();
    let mut response =
        client.get(ovgu_mensa_unten!()).send().expect("OVGU website is unreachable!");
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
                            .and_then(|node| node.text().next()) {
                            Some(v) => v.trim(),
                            None => "",
                        }
                        .split_whitespace()
                        .take_while(|item| !item.is_empty());

                    let mut rest = vec![];
                    let additives = notes.filter_map(|item| {
                            match Additive::from_str(item) {
                                Ok(v) => Some(v),
                                Err(..) => {
                                    rest.push(item);
                                    None
                                }
                            }
                        })
                        .collect();
                    let allergenics = rest.iter()
                        .map(|item| {
                            Allergenic::from_str(item)
                                .expect(format!("Cannot parse note '{}'!", item).as_str())
                        })
                        .collect();

                    let name = element.select(&sel_name)
                        .next()
                        .and_then(|node| node.text().next())
                        .expect("No meal name found!")
                        .trim();

                    let price = Price::from_str(element.select(&sel_price)
                            .next()
                            .and_then(|node| node.text().last())
                            .expect("No price found!")
                            .trim())
                        .unwrap();

                    let symbols = element.select(&sel_symbols)
                        .map(|img| {
                            Symbol::from_str(img.value()
                                    .attr("title")
                                    .expect("Symbol tag has no title!")
                                    .trim())
                                .unwrap()
                        })
                        .collect();

                    Meal {
                        name: name,
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
            let side_dishes = side_dishes_str[10..].trim().split(", ").collect::<Vec<&str>>();

            Day {
                date: date,
                meals: meals,
                side_dishes: side_dishes,
            }
        })
        .collect::<Vec<Day>>();

    println!("{:?}", days);
    // println!("{}", serde_json::to_string(&days).expect("Failed to convert to JSON!"));
}
