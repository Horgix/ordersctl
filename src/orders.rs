use prettytable::Table;
use serde::Deserialize;
use std::fmt;
use std::{fs::File, io::BufReader, path::Path};

use std::error::Error;


use crate::providers::Providers;
use crate::status::Status;
use crate::{cost::Cost, status::BoolRepr};

#[derive(Debug, Deserialize)]
pub struct Order {
    pub description: String,
    pub provider: Providers,
    pub content: Vec<String>,
    pub cost: Cost,
    pub status: Status,
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
