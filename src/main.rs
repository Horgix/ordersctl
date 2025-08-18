#[macro_use]
extern crate prettytable;

pub mod order;
pub mod orders;
pub mod status;

fn main() {
    println!("Hello, world!");

    let orders = orders::read_orders_from_file("./samples/actual-horgix-orders.yml");
    match orders {
        Ok(orders) => println!("Successfully read orders from file: \n{}", orders),
        Err(e) => println!("Error reading orders: {}", e),
    }
}
