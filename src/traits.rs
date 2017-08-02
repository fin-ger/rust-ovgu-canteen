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

use scraper;

/// This trait is used to create an instance from a HTML element reference.
pub trait FromElement: Sized {
    /// This is the error type used when the creation from a HTML element fails.
    type Err;

    /// Create an instance of `Self` from a HTML element reference.
    ///
    /// # Arguments
    ///
    /// `e`  - The HTML element reference.
    ///
    /// # Returns
    ///
    /// A result containing an instance of `Self` or an error.
    fn from_element(e: &scraper::ElementRef) -> Result<Self, Self::Err>;
}

/// This trait is used to update the data of an instance.
pub trait Update: Sized {
    /// This is the error type used when the update fails.
    type Err;

    /// Update this instance from another instance with the same type.
    ///
    /// # Arguments
    ///
    /// `from`  - The other instance that is used to update this instance.
    ///
    /// # Returns
    ///
    /// A result containing an error that may occur during the update.
    fn update(&mut self, from: &Self) -> Result<(), Self::Err>;
}
