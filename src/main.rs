#[macro_use] extern crate prettytable;

pub mod orders;
pub mod cost;
pub mod status;
pub mod tests;

fn main() {
    println!("Hello, world!");

    let orders = orders::read_orders_from_file("/home/horgix/projects/ordersctl/samples/orders.yml").unwrap();
    println!("Deserialized the following Oders: \n{}", orders);
}