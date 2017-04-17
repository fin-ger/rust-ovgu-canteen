use std;

#[derive(Debug)]
pub enum Error
{
    AdditiveCreation(String),
    AllergenicCreation(String),
    SymbolCreation(String),
    NoPrice(String),
    InvalidPrice(String, std::num::ParseFloatError),
}

impl std::fmt::Display for Error
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match *self
        {
            Error::AdditiveCreation(ref input) => write!(f, "Error creating Additive from '{}'", input),
            Error::AllergenicCreation(ref input) => write!(f, "Error creating Allergenic from '{}'", input),
            Error::SymbolCreation(ref input) => write!(f, "Error creating Symbol from '{}'", input),
            Error::NoPrice(ref what) => write!(f, "Error finding Price for {}", what),
            Error::InvalidPrice(ref what, ref e) => write!(f, "Error parsing Price '{}' - Reason: {}", what, e),
        }
    }
}

impl std::error::Error for Error
{
    fn description(&self) -> &str
    {
        match *self
        {
            Error::AdditiveCreation(..) => "Cannot create additive",
            Error::AllergenicCreation(..) => "Cannot create allergenic",
            Error::SymbolCreation(..) => "Cannot create symbol",
            Error::NoPrice(..) => "Cannot find price",
            Error::InvalidPrice(..) => "Cannot parse price",
        }
    }
}
