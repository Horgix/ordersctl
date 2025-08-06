#[macro_use]
extern crate prettytable;

pub mod order;
pub mod orders;
pub mod status;

fn main() {
    println!("Hello, world!");

    let orders =
        orders::read_orders_from_file("./samples/orders.yml")
            .unwrap();
    println!("Deserialized the following Oders: \n{}", orders);
}
