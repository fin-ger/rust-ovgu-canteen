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
    [uni_campus_lower_hall] => ( base_url!["mensa-unicampus/speiseplan-unten/"] );
    [uni_campus_upper_hall] => ( base_url!["mensa-unicampus/speiseplan-oben/"] );
    [kellercafe] => ( base_url!["mensa-kellercafe/speiseplan/"] );
    [herrenkrug] => ( base_url!["mensa-herrenkrug/speiseplan/"] );
    [stendal] => ( base_url!["mensa-stendal/speiseplan/"] );
    [wernigerode] => ( base_url!["mensa-wernigerode/speiseplan/"] );
    [dom_cafete_halberstadt] => ( base_url!["mensa-halberstadt/speiseplan/"] );
}

macro_rules! parse_and_unwrap_selector {
    ($sel:tt) => ( ::scraper::Selector::parse($sel).unwrap() )
}

#[macro_export]
macro_rules! ovgu_canteen_selector {
    [day] => ( parse_and_unwrap_selector!(".mensa > table") );
    [date] => ( parse_and_unwrap_selector!("thead > tr > td") );
    [side_dishes] => ( parse_and_unwrap_selector!("tbody > tr:last-of-type > td[colspan=\"3\"]") );
    [meal] => ( parse_and_unwrap_selector!("tbody > tr") );
    [name] => ( parse_and_unwrap_selector!("td:nth-of-type(1) > strong") );
    [price] => ( parse_and_unwrap_selector!("td:nth-of-type(1)") );
    [symbols] => ( parse_and_unwrap_selector!("td:nth-of-type(2) > div > img") );
    [notes] => ( parse_and_unwrap_selector!("td:nth-of-type(2) > div:nth-of-type(2)") );
}
