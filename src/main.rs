use std::env;

#[macro_use]
extern crate prettytable;

pub mod order;
pub mod orders;
pub mod status;

fn main() {
    let orders_file =
        env::var("ORDERSCTL_ORDERS_FILE").unwrap_or("./samples/orders.yml".to_string());

    let orders = orders::read_orders_from_file(orders_file);
    match orders {
        Ok(orders) => println!("Successfully read orders from file: \n{}", orders),
        Err(e) => println!("Error reading orders: {}", e),
    }
}
