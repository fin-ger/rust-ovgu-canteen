use chrono;
use scraper;
use ovgu;
use ovgu::canteen::Meal;

#[derive(Serialize, Deserialize)]
pub struct Day
{
    pub date: chrono::NaiveDate,
    pub meals: Vec<Meal>,
    pub side_dishes: Vec<String>,
}

impl ovgu::canteen::FromElement for Day
{
    type Err = ovgu::Error;
    fn from_element(day_node: &scraper::ElementRef) -> Result<Self, Self::Err>
    {
        let date = day_node.select(&ovgu_canteen_selector![date])
            .next()
            .and_then(|node| node.text().next())
            .ok_or(ovgu::Error::NotAvailable("date", "day", None))
            .and_then(|date_str| {
                chrono::NaiveDate::parse_from_str(
                    &date_str[date_str.len() - 10..],
                    "%d.%m.%Y"
                ).map_err(|e| ovgu::Error::InvalidValue("date", "day", Some(Box::new(e))))
            })?;

        // we create meals from a given html node
        // then we collect an Iter<Result<Meal, Err>> into a Result<Vec<Meal>, Err>
        //  -> collect checks if any of the results failed
        //  -> therefore we can use ? on the collected Vec
        let meals = day_node.select(&ovgu_canteen_selector![meal])
            .map(|meal_node| Meal::from_element(&meal_node))
            .collect::<Result<Vec<Meal>, ovgu::Error>>()?;

        let side_dishes = day_node.select(&ovgu_canteen_selector![side_dishes])
            .next()
            .and_then(|node| node.text().next())
            .ok_or(ovgu::Error::NotAvailable("side_dishes", "day", None))
            .map(|side_dishes_str| {
                side_dishes_str[10..].trim()
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect()
            })?;

        Ok(
            Day
            {
                date: date,
                meals: meals,
                side_dishes: side_dishes,
            }
        )
    }
}
