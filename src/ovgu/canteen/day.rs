use chrono;
use ovgu::canteen::Meal;

#[derive(Serialize, Deserialize)]
pub struct Day
{
    pub date: chrono::NaiveDate,
    pub meals: Vec<Meal>,
    pub side_dishes: Vec<String>,
}
