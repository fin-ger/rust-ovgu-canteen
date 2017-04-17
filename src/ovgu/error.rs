use std;

#[derive(Debug)]
pub enum Error
{
    Creation(&'static str, String, Option<Box<std::error::Error>>),
    NotAvailable(&'static str, &'static str, Option<Box<std::error::Error>>),
    InvalidValue(&'static str, &'static str, Option<Box<std::error::Error>>),
}

impl std::fmt::Display for Error
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match *self
        {
            Error::Creation(ref what, ref input, ref err) => {
                match *err
                {
                    Some(ref err_box) => {
                        write!(f, "Error creating {} from '{}' - Reason: {}", what, input, err_box)
                    },
                    None => {
                        write!(f, "Error creating {} from '{}'!", what, input)
                    }
                }
            },
            Error::NotAvailable(ref what, ref thing, ref err) => {
                match *err
                {
                    Some(ref err_box) => {
                        write!(f, "Error {} is unavailable for {} - Reason: {}", what, thing, err_box)
                    },
                    None => {
                        write!(f, "Error {} is unavailable for {}!", what, thing)
                    }
                }
            },
            Error::InvalidValue(ref what, ref val, ref err) => {
                match *err
                {
                    Some(ref err_box) => {
                        write!(f, "Error parsing {} '{}' - Reason: {}", what, val, err_box)
                    },
                    None => {
                        write!(f, "Error parsing {} '{}'!", what, val)
                    }
                }
            },
        }
    }
}

impl std::error::Error for Error
{
    fn description(&self) -> &str
    {
        match *self
        {
            Error::Creation(..) => "Cannot create instance",
            Error::NotAvailable(..) => "Cannot find requested value",
            Error::InvalidValue(..) => "Cannot parse given value",
        }
    }
}
