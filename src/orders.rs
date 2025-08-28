use std::error::Error;
use std::fmt;
use std::{fs::File, io::BufReader, path::Path};

use itertools::Itertools;
use prettytable::Table;
use rusty_money::Money;
use serde::Deserialize;

use crate::order::Order;
use crate::status::BoolRepr;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Orders {
    Object(Vec<Order>),
}

pub struct OrderStats {
    pub total_orders: usize,
    pub total_cost: Money<'static, rusty_money::iso::Currency>,
    pub by_status: std::collections::HashMap<&'static str, usize>,
}

impl Orders {
    pub fn stats(&self) -> OrderStats {
        match self {
            Orders::Object(orders) => {
                let total_orders = orders.len();
                // Get a map with the orders by status
                let orders_by_status = std::collections::HashMap::from([
                    (
                        "confirmed",
                        orders.iter().filter(|o| o.status.confirmed).count(),
                    ),
                    ("paid", orders.iter().filter(|o| o.status.paid).count()),
                    (
                        "shipped",
                        orders.iter().filter(|o| o.status.shipped).count(),
                    ),
                    (
                        "received",
                        orders.iter().filter(|o| o.status.received).count(),
                    ),
                ]);

                // FIXME: For now it considers all currencies the same
                let total_cost = orders
                    .iter()
                    // Init with zero from rusty_money
                    .fold(Money::from_minor(0, rusty_money::iso::EUR), |acc, order| {
                        acc + order.cost.clone()
                    });

                OrderStats {
                    total_orders,
                    total_cost,
                    by_status: orders_by_status,
                }
            }
        }
    }
}

pub fn read_orders_from_file<P: AsRef<Path>>(path: P) -> Result<Orders, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let orders = serde_yaml::from_reader(reader)?;

    Ok(orders)
}

impl fmt::Debug for Orders {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Orders::Object(orders) => {
                for order in orders {
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
        // Formatted stats in a single line
        let stats_foote = format!(
            "Total: {} Orders ({}) | {} confirmed, {} paid, {} shipped, {} received",
            self.stats().total_orders,
            self.stats().total_cost,
            self.stats().by_status.get("confirmed").unwrap(),
            self.stats().by_status.get("paid").unwrap(),
            self.stats().by_status.get("shipped").unwrap(),
            self.stats().by_status.get("received").unwrap(),
        );
        return write!(f, "{}\n{}", table.to_string(), stats_foote);
    }
}
