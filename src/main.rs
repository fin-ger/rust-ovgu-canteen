#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate hyper_native_tls;
extern crate scraper;
extern crate html5ever;
extern crate chrono;
extern crate serde;
extern crate serde_json;

use chrono::Datelike;
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

#[derive(Serialize, Deserialize, Debug)]
enum Symbol
{
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

impl FromStr for Symbol
{
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String>
    {
        match s
        {
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

#[derive(Serialize, Deserialize, Debug)]
enum Additive
{
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

impl FromStr for Additive
{
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String>
    {
        match s
        {
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

#[derive(Serialize, Deserialize, Debug)]
enum Allergenic
{
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

impl FromStr for Allergenic
{
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String>
    {
        match s
        {
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

#[derive(Serialize, Deserialize, Debug)]
struct Price
{
    student: f32,
    staff: f32,
    guest: f32,
}

impl FromStr for Price
{
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String>
    {
        let replaced = str::replace(s, ",", ".");
        let mut split = replaced.split(" | ");

        let student = split.next()
            .ok_or("Cannot find student price!")
            .and_then(|num| num.parse().map_err(|_| "Failed to parse student price!"))
            ?;

        let staff = split.next()
            .ok_or("Cannot find staff price!")
            .and_then(|num| num.parse().map_err(|_| "Failed to parse staff price!"))
            ?;

        let guest = split.next()
            .ok_or("Cannot find guest price!")
            .and_then(|num| num.parse().map_err(|_| "Failed to parse guest price!"))
            ?;

        Ok(Price {
            student: student,
            staff: staff,
            guest: guest,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Meal
{
    name: String,
    price: Price,
    symbols: Vec<Symbol>,
    additives: Vec<Additive>,
    allergenics: Vec<Allergenic>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Day
{
    date: NaiveDateSerde,
    meals: Vec<Meal>,
    side_dishes: Vec<String>,
}

struct NaiveDateVisitor {}

impl NaiveDateVisitor
{
    fn new() -> Self
    {
        NaiveDateVisitor {}
    }
}

#[derive(Debug)]
struct NaiveDateSerde(chrono::NaiveDate);

impl serde::Deserialize for NaiveDateSerde
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer
    {
        deserializer.deserialize_i32(NaiveDateVisitor::new())
    }
}

impl serde::Serialize for NaiveDateSerde
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        serializer.serialize_i32(self.num_days_from_ce())
    }
}

impl serde::de::Visitor for NaiveDateVisitor
{
    type Value = NaiveDateSerde;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        formatter.write_str(concat!(
            "an i32 integer describing a date with the number ",
            "of days since 0001-01-01 in the proleptic Gregorian calendar"
        ))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
        where E: serde::de::Error
    {
        match chrono::NaiveDate::from_num_days_from_ce_opt(v)
        {
            Some(d) => Ok(NaiveDateSerde(d)),
            None =>
            {
                let unexpected = serde::de::Unexpected::Signed(v as i64);
                Err(serde::de::Error::invalid_value(unexpected, &self))
            }
        }
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
        where E: serde::de::Error
    {
        Ok(NaiveDateSerde(chrono::NaiveDate::from_num_days_from_ce(1)))
    }
}

impl std::ops::Deref for NaiveDateSerde
{
    type Target = chrono::NaiveDate;
    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}

fn main()
{
    let ssl = hyper_native_tls::NativeTlsClient::new().expect("Cannot create TLS client!");
    let connector = hyper::net::HttpsConnector::new(ssl);
    let client = hyper::Client::with_connector(connector);
    let mut body = String::new();
    let mut response = client.get(ovgu_mensa_unten!())
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
                            match Additive::from_str(item)
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

            Day {
                date: NaiveDateSerde(date),
                meals: meals,
                side_dishes: side_dishes,
            }
        })
        .collect::<Vec<Day>>();

    serde_json::to_writer_pretty(&mut std::io::stdout(), &days).unwrap();
    println!();
}
