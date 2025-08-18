use std::error::Error;
use std::fmt;
use std::{fs::File, io::BufReader, path::Path};

use itertools::Itertools;
use prettytable::Table;
use serde::Deserialize;

use crate::order::Order;
use crate::status::BoolRepr;

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
            "Order ID",
            "Date",
            "Cost",
            "Confirmed",
            "Paid",
            "Shipped",
            "Received"
        ]);
        match self {
            Orders::Object(orders) => {
                // Truncate the description to 30 characters and add ellipsis  (...) if necessary
                let truncate_description_if_needed = |desc: &str| {
                    if desc.len() > 30 {
                        format!("{}...", &desc[..27])
                    } else {
                        desc.to_string()
                    }
                };
                for order in orders.into_iter().sorted_by_key(|o| o.date) {
                    table.add_row(row![
                    truncate_description_if_needed(&order.description),
                    order.provider,
                    order.id,
                    order.date,
                    order.cost,
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
