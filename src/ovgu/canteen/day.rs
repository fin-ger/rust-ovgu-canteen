use ovgu::canteen;

#[derive(Serialize, Deserialize)]
pub struct Day
{
    pub date: canteen::NaiveDateSerde,
    pub meals: Vec<canteen::Meal>,
    pub side_dishes: Vec<String>,
}
