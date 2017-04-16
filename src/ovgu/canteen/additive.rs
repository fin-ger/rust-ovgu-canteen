use std;

#[derive(Serialize, Deserialize)]
pub enum Additive
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

impl std::str::FromStr for Additive
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
