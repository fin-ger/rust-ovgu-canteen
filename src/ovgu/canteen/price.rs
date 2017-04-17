use std;
use ovgu;

#[derive(Serialize, Deserialize)]
pub struct Price
{
    pub student: f32,
    pub staff: f32,
    pub guest: f32,
}

impl std::str::FromStr for Price
{
    type Err = ovgu::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let replaced = str::replace(s, ",", ".");
        let mut split = replaced.split(" | ");

        let price_student = split.next()
            .ok_or(ovgu::Error::NoPrice("student".to_owned()))
            .and_then(|num| {
                num.parse()
                    .map_err(|e| ovgu::Error::InvalidPrice("student".to_owned(), e))
            })?;

        let price_staff = split.next()
            .ok_or(ovgu::Error::NoPrice("staff".to_owned()))
            .and_then(|num| {
                num.parse()
                    .map_err(|e| ovgu::Error::InvalidPrice("staff".to_owned(), e))
            })?;

        let price_guest = split.next()
            .ok_or(ovgu::Error::NoPrice("guest".to_owned()))
            .and_then(|num| {
                num.parse()
                    .map_err(|e| ovgu::Error::InvalidPrice("guest".to_owned(), e))
            })?;

        Ok(ovgu::canteen::Price {
            student: price_student,
            staff: price_staff,
            guest: price_guest,
        })
    }
}
