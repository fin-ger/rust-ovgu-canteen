use std;
use ovgu;

#[derive(Serialize, Deserialize)]
pub enum Allergenic
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

impl std::str::FromStr for Allergenic
{
    type Err = ovgu::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err>
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
            _ => Err(ovgu::Error::AllergenicCreation(s.to_owned())),
        }
    }
}
