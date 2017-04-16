use ovgu::canteen;

#[derive(Serialize, Deserialize)]
pub struct Meal
{
    pub name: String,
    pub price: canteen::Price,
    pub symbols: Vec<canteen::Symbol>,
    pub additives: Vec<canteen::Additive>,
    pub allergenics: Vec<canteen::Allergenic>,
}
