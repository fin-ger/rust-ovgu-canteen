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

macro_rules! base_url {
    [$sub:tt] => {
        concat!("https://www.studentenwerk-magdeburg.de/mensen-cafeterien/", $sub)
    }
}

#[macro_export]
macro_rules! ovgu_canteen_url {
    [downstairs] => ( base_url!["mensa-unicampus/speiseplan-unten/"] );
    [upstairs] => ( base_url!["mensa-unicampus/speiseplan-oben/"] );
}

macro_rules! parse_and_unwrap_selector {
    ($sel:tt) => ( ::scraper::Selector::parse($sel).unwrap() )
}

#[macro_export]
macro_rules! ovgu_canteen_selector {
    [day] => ( parse_and_unwrap_selector!(".mensa > table") );
    [date] => ( parse_and_unwrap_selector!("thead > tr > td") );
    [side_dishes] => ( parse_and_unwrap_selector!("tbody > tr:last-of-type > td") );
    [meal] => ( parse_and_unwrap_selector!("tbody > tr:not(:last-of-type)") );
    [name] => ( parse_and_unwrap_selector!("td:nth-of-type(1) > strong") );
    [price] => ( parse_and_unwrap_selector!("td:nth-of-type(1)") );
    [symbols] => ( parse_and_unwrap_selector!("td:nth-of-type(2) > div > img") );
    [notes] => ( parse_and_unwrap_selector!("td:nth-of-type(2) > div:nth-of-type(2)") );
}
