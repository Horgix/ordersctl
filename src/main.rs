#[macro_use] extern crate prettytable;

use std::{fs::File, io::BufReader, path::Path};

use std::error::Error;

pub mod providers;
pub mod orders;
pub mod cost;
pub mod status;

fn read_orders_from_file<P: AsRef<Path>>(path: P) -> Result<orders::Orders, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let order: orders::Orders = serde_yaml::from_reader(reader)?;

    // Return the `User`.
    Ok(order)
}

fn main() {
    println!("Hello, world!");

    let order = read_orders_from_file("/home/horgix/projects/ordersctl/samples/orders.yml").unwrap();
    println!("Deserialized the following Oders: \n{}", order);
}