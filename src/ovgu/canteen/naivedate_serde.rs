use std;
use serde;
use chrono;
use chrono::Datelike;

struct NaiveDateVisitor;

impl NaiveDateVisitor
{
    fn new() -> Self
    {
        NaiveDateVisitor
    }
}

pub struct NaiveDateSerde(pub chrono::NaiveDate);

impl serde::Deserialize for NaiveDateSerde
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer
    {
        deserializer.deserialize_i32(NaiveDateVisitor::new())
    }
}

impl serde::Serialize for NaiveDateSerde
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        serializer.serialize_i32(self.num_days_from_ce())
    }
}

impl serde::de::Visitor for NaiveDateVisitor
{
    type Value = NaiveDateSerde;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        formatter.write_str("an i32 integer describing a date with the number \
                             of days since 0001-01-01 in the proleptic Gregorian calendar")
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
        where E: serde::de::Error
    {
        match chrono::NaiveDate::from_num_days_from_ce_opt(v)
        {
            Some(d) => Ok(NaiveDateSerde(d)),
            None =>
            {
                let unexpected = serde::de::Unexpected::Signed(v as i64);
                Err(serde::de::Error::invalid_value(unexpected, &self))
            }
        }
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
        where E: serde::de::Error
    {
        Ok(NaiveDateSerde(chrono::NaiveDate::from_num_days_from_ce(1)))
    }
}

impl std::ops::Deref for NaiveDateSerde
{
    type Target = chrono::NaiveDate;

    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}
