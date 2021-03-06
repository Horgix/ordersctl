use chrono::NaiveDate;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use rusty_money::{iso, Money};
use serde::de::{self};
use serde::{Deserialize, Deserializer};
use std::fmt;

use crate::status::Status;

#[derive(Debug, Deserialize)]
pub struct Order {
    pub description: String,
    pub provider: String,
    pub id: String,
    #[serde(deserialize_with = "deserialize_naivedate")]
    pub date: NaiveDate,
    pub content: Vec<String>,
    #[serde(deserialize_with = "deserialize_iso_4217_money")]
    pub cost: Money<'static, iso::Currency>,
    pub status: Status,
}

pub fn deserialize_naivedate<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<NaiveDate, D::Error> {
    let time: String = Deserialize::deserialize(deserializer)?;
    Ok(NaiveDate::parse_from_str(&time, "%Y-%m-%d").unwrap())
}

struct DeserializeISO4217MoneyVisitor;

impl<'de> de::Visitor<'de> for DeserializeISO4217MoneyVisitor {
    type Value = Money<'static, iso::Currency>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("TODO")
    }

    fn visit_f64<Error>(self, v: f64) -> Result<Self::Value, Error> {
        Ok(Money::from_decimal(Decimal::from_f64(v).unwrap(), iso::EUR))
    }

    fn visit_i64<Error>(self, v: i64) -> Result<Self::Value, Error> {
        Ok(Money::from_decimal(Decimal::from_i64(v).unwrap(), iso::EUR))
    }

    fn visit_u64<Error>(self, v: u64) -> Result<Self::Value, Error> {
        Ok(Money::from_decimal(Decimal::from_u64(v).unwrap(), iso::EUR))
    }
}

fn deserialize_iso_4217_money<'de, D>(
    deserializer: D,
) -> Result<Money<'static, iso::Currency>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(DeserializeISO4217MoneyVisitor)
}
