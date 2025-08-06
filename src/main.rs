use std::error::Error;
use std::io::stdin;

use tokio;

pub mod date_utils;
pub mod enums;
pub mod ethereum;
pub mod structs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, which chain is your wallet from?");
    println!("1. Ethereum");
    println!("2. Solana");

    let mut choice = String::new();
    loop {
        choice.clear();
        stdin().read_line(&mut choice)?;
        match choice.trim() {
            "1" => {
                ethereum::main().await;
                break;
            }
            "2" => {
                println!("To be implemented");
                break;
            }
            _ => println!("Please insert a valid option"),
        }
    }

    Ok(())
}
