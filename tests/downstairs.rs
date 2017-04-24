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

extern crate ovgu_canteen;
extern crate chrono;

use ovgu_canteen::ovgu::canteen::{Canteen, CanteenDescription};
use chrono::TimeZone;

#[test]
fn canteen_downstairs()
{
    // test if parsing is working
    let canteen = Canteen::new(CanteenDescription::Downstairs).unwrap();

    assert_eq!(canteen.description, CanteenDescription::Downstairs);
    assert!(canteen.days.len() > 0);

    for day in canteen.days
    {
        // this is not quite correct as the local timezone may not be summer time aware MEZ
        assert!(chrono::Local::today() <= chrono::Local.from_local_date(&day.date).unwrap());

        for meal in day.meals
        {
            assert!(meal.name.len() > 0);

            assert!(0f32 < meal.price.student);
            assert!(meal.price.student <= meal.price.staff);
            assert!(meal.price.staff <= meal.price.guest);
        }

        for side_dish in day.side_dishes
        {
            assert!(side_dish.len() > 0);
        }
    }
}
