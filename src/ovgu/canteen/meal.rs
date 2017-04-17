use ovgu::canteen::{Price, Symbol, Additive, Allergenic};

#[derive(Serialize, Deserialize)]
pub struct Meal
{
    pub name: String,
    pub price: Price,
    pub symbols: Vec<Symbol>,
    pub additives: Vec<Additive>,
    pub allergenics: Vec<Allergenic>,
}
