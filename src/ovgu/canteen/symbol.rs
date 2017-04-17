
use ovgu;
use std;

#[derive(Serialize, Deserialize)]
pub enum Symbol
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

impl std::str::FromStr for Symbol
{
    type Err = ovgu::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err>
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
            _ => Err(ovgu::Error::Creation("symbol", s.to_owned(), None)),
        }
    }
}
