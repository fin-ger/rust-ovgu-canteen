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
            .ok_or(ovgu::Error::NotAvailable("price", "student", None))
            .and_then(|num| {
                num.parse::<f32>()
                    .map_err(|e| ovgu::Error::InvalidValue("price", "student", Some(Box::new(e))))
            })?;

        let price_staff = split.next()
            .ok_or(ovgu::Error::NotAvailable("price", "staff", None))
            .and_then(|num| {
                num.parse::<f32>()
                    .map_err(|e| ovgu::Error::InvalidValue("price", "staff", Some(Box::new(e))))
            })?;

        let price_guest = split.next()
            .ok_or(ovgu::Error::NotAvailable("price", "guest", None))
            .and_then(|num| {
                num.parse::<f32>()
                    .map_err(|e| ovgu::Error::InvalidValue("price", "guest", Some(Box::new(e))))
            })?;

        Ok(ovgu::canteen::Price {
            student: price_student,
            staff: price_staff,
            guest: price_guest,
        })
    }
}
