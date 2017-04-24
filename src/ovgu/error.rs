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
