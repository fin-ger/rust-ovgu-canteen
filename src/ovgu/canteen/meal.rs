use scraper;
use ovgu;
use ovgu::canteen::{Price, Symbol, Additive, Allergenic};
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
pub struct Meal
{
    pub name: String,
    pub price: Price,
    pub symbols: Vec<Symbol>,
    pub additives: Vec<Additive>,
    pub allergenics: Vec<Allergenic>,
}

impl ovgu::canteen::FromElement for Meal
{
    type Err = ovgu::Error;
    fn from_element(meal_node: &scraper::ElementRef) -> Result<Self, Self::Err>
    {
        let notes = meal_node.select(&ovgu_canteen_selector![notes])
            .next()
            .and_then(|node| node.text().next())
            .unwrap_or("")
            .trim()
            .split_whitespace()
            .filter(|item| !item.is_empty());

        let mut rest = vec![];
        let additives = notes
            .filter_map(|item| {
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
            .map(|item| Allergenic::from_str(item))
            .collect::<Result<Vec<Allergenic>, ovgu::Error>>()?;

        let name = meal_node.select(&ovgu_canteen_selector![name])
            .next()
            .and_then(|node| node.text().next())
            .ok_or(ovgu::Error::NotAvailable("name", "meal", None))
            .map(|n| n.trim())?;

        let price = meal_node.select(&ovgu_canteen_selector![price])
            .next()
            .and_then(|node| node.text().last())
            .ok_or(ovgu::Error::NotAvailable("price", "meal", None))
            .and_then(|p| Price::from_str(p.trim()))?;

        let symbols = meal_node.select(&ovgu_canteen_selector![symbols])
            .map(|img| {
                img.value()
                    .attr("title")
                    .ok_or(ovgu::Error::NotAvailable("title", "symbol img tag", None))
                    .and_then(|t| ovgu::canteen::Symbol::from_str(t.trim()))
            })
            .collect::<Result<Vec<Symbol>, ovgu::Error>>()?;

        Ok(
            Meal
            {
                name: name.to_owned(),
                price: price,
                symbols: symbols,
                additives: additives,
                allergenics: allergenics,
            }
        )
    }
}
