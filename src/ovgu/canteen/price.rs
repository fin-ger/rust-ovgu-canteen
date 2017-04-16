use std;
use ovgu::canteen;

#[derive(Serialize, Deserialize)]
pub struct Price
{
    pub student: f32,
    pub staff: f32,
    pub guest: f32,
}

impl std::str::FromStr for Price
{
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String>
    {
        let replaced = str::replace(s, ",", ".");
        let mut split = replaced.split(" | ");

        let student = split.next()
            .ok_or("Cannot find student price!")
            .and_then(|num| num.parse().map_err(|_| "Failed to parse student price!"))
            ?;

        let staff = split.next()
            .ok_or("Cannot find staff price!")
            .and_then(|num| num.parse().map_err(|_| "Failed to parse staff price!"))
            ?;

        let guest = split.next()
            .ok_or("Cannot find guest price!")
            .and_then(|num| num.parse().map_err(|_| "Failed to parse guest price!"))
            ?;

        Ok(canteen::Price {
            student: student,
            staff: staff,
            guest: guest,
        })
    }
}
