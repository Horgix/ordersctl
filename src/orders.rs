use rust_decimal::prelude::FromPrimitive;
use rusty_money::{Money, iso};
extern crate rusty_money;
extern crate rust_decimal;
use prettytable::Table;
  
use std::fmt;
use std::{fs::File, io::BufReader, path::Path};
use std::error::Error;

use serde::{Deserialize, Deserializer};
use serde::de::{self};

use crate::status::Status;
use crate::{status::BoolRepr};

#[derive(Debug, Deserialize)]
pub struct Order {
    pub description: String,
    pub provider: String,
    pub content: Vec<String>,
    #[serde(deserialize_with = "deserialize_iso_4217_money")]
    pub cost: Money<'static, iso::Currency>,
    pub status: Status,
}

struct DeserializeISO4217MoneyVisitor;

impl<'de> de::Visitor<'de> for DeserializeISO4217MoneyVisitor {
    type Value = Money<'static, iso::Currency>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("TODO")
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
            E: Error, {
        Ok(Money::from_decimal(rust_decimal::Decimal::from_f64(v).unwrap(), rusty_money::iso::EUR))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
            E: Error, {
        Ok(Money::from_decimal(rust_decimal::Decimal::from_i64(v).unwrap(), rusty_money::iso::EUR))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
            E: Error, {
        Ok(Money::from_decimal(rust_decimal::Decimal::from_u64(v).unwrap(), rusty_money::iso::EUR))
    }
}

fn deserialize_iso_4217_money<'de, D>(deserializer: D) -> Result<Money<'static, iso::Currency>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(DeserializeISO4217MoneyVisitor)
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Orders {
    Object(Vec<Order>),
}
pub fn read_orders_from_file<P: AsRef<Path>>(path: P) -> Result<Orders, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let orders = serde_yaml::from_reader(reader)?;

    // Return the `User`.
    Ok(orders)
}

impl fmt::Debug for Orders {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Orders::Object(orders) => {
                for order in orders {
                    // TODO fix unused write! std::result::Result
                    // Construct result string then write it?
                    match writeln!(f, "- {:?}", order) {
                        Ok(_) => {}
                        Err(e) => {
                            log::error!("Failed to format an Order: {:?}", e);
                        }
                    }
                }
                return writeln!(f);
            }
        }
    }
}

impl fmt::Display for Orders {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Create the table
        let mut table = Table::new();

        table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        table.set_titles(row![
            "Description",
            "Provider",
            "Confirmed",
            "Paid",
            "Shipped",
            "Received"
        ]);
        match self {
            Orders::Object(orders) => {
                for order in orders {
                    table.add_row(row![
                    order.description,
                    order.provider,
                    c->order.status.confirmed.to_utf8_colored(),
                    c->order.status.paid.to_utf8_colored(),
                    c->order.status.shipped.to_utf8_colored(),
                    c->order.status.received.to_utf8_colored(),
                    ]);
                }
            }
        }
        // Print the table to stdout
        return write!(f, "{}", table.to_string());
    }
}
