// libovgu-canteen - A canteen parser module for ovgu.
//
// Copyright (C) 2017
//     Fin Christensen <christensen.fin@gmail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std;

/// The `Error` enum represents several different error types that are used
/// by results in this crate.
#[derive(Debug)]
pub enum Error
{
    /// This error is used when a creation of something failed.
    ///
    ///  * The first parameter is used to define *what* failed to create.
    ///  * The second parameter is used to provide a string representation
    ///    from *which* the creation failed.
    ///  * The third parameter contains the previous error.
    ///
    /// # Examples
    ///
    /// ```
    /// use ovgu_canteen::ovgu::Error;
    ///
    /// let data = String::from("five");
    /// let number = match data.as_str()
    /// {
    ///     "five" => Ok(5),
    ///     _ => Err(Error::Creation("number", data, None)),
    /// };
    /// ```
    Creation(&'static str, String, Option<Box<std::error::Error>>),

    /// This error is used when something is not available or cannot be found.
    ///
    ///  * The first parameter is used to define what is *affected* by this error.
    ///  * The second parameter is used to define what *item* is not available.
    ///  * The third parameter contains the previous error.
    ///
    /// # Examples
    ///
    /// ```
    /// use ovgu_canteen::ovgu::Error;
    ///
    /// let data = ["foo"];
    /// let mut iter = data.iter();
    /// let result = iter.next().ok_or(Error::NotAvailable("result", "foo", None));
    /// ```
    NotAvailable(&'static str, &'static str, Option<Box<std::error::Error>>),

    /// This error is used when invalid data got passed.
    ///
    ///  * The first parameter is used to define what is *affected* by this error.
    ///  * The second parameter is used to define what *item* is invalid.
    ///  * The third parameter contains the previous error.
    ///
    /// # Examples
    ///
    /// ```
    /// use ovgu_canteen::ovgu::Error;
    ///
    /// let data = String::from("42");
    /// let number = data.parse::<f32>().map_err(|e| Error::InvalidValue("number", "data", Some(Box::new(e))));
    /// ```
    InvalidValue(&'static str, &'static str, Option<Box<std::error::Error>>),
}

impl std::fmt::Display for Error
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match *self
        {
            Error::Creation(ref what, ref input, ref err) =>
            {
                match *err
                {
                    Some(ref err_box) =>
                    {
                        write!(f,
                               "Error creating {} from '{}' - Reason: {}",
                               what,
                               input,
                               err_box)
                    }
                    None => write!(f, "Error creating {} from '{}'!", what, input),
                }
            }
            Error::NotAvailable(ref what, ref thing, ref err) =>
            {
                match *err
                {
                    Some(ref err_box) =>
                    {
                        write!(f,
                               "Error {} is unavailable for {} - Reason: {}",
                               what,
                               thing,
                               err_box)
                    }
                    None => write!(f, "Error {} is unavailable for {}!", what, thing),
                }
            }
            Error::InvalidValue(ref what, ref val, ref err) =>
            {
                match *err
                {
                    Some(ref err_box) =>
                    {
                        write!(f, "Error parsing {} '{}' - Reason: {}", what, val, err_box)
                    }
                    None => write!(f, "Error parsing {} '{}'!", what, val),
                }
            }
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
