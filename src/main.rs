use std::env;
use clap::{Parser, Subcommand};

#[macro_use]
extern crate prettytable;

pub mod order;
pub mod orders;
pub mod status;

/// Main CLI structure
#[derive(Parser, Debug)]
#[command(
    name = "ordersctl",
    version = "0.1",
    author = "Alexis Horgix Chotard <ordersctl@foss.horgix.fr>",
    about = " OrdersCtl — A simple CLI to track your orders",
)]
struct OrdersCtlCli {
    /// Subcommands
    #[command(subcommand)]
    command: Option<OrdersCtlSubcommands>,
}

/// Top-level subcommands
#[derive(Subcommand, Debug)]
enum OrdersCtlSubcommands {
    /// List all recordings (and their transcription statuses TODO)
    List {},
}

fn main() {
    let orders_file =
        env::var("ORDERSCTL_ORDERS_FILE").unwrap_or("./samples/orders.yml".to_string());

    let orders = orders::read_orders_from_file(orders_file);


    let cli = OrdersCtlCli::parse();

    match &cli.command.unwrap_or(OrdersCtlSubcommands::List {}) {
        OrdersCtlSubcommands::List {} => {
            match orders {
                Ok(orders) => println!("Successfully read orders from file: \n{}", orders),
                Err(e) => println!("Error reading orders: {}", e),
            }
        }
    }
}
