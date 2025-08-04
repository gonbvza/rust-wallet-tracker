//
// This is the entry point for the cli
// From here the code diverges depending on the chain
//

use std::io::stdin;
use tokio;

pub mod date_utils;
pub mod enums;
pub mod ethereum;
pub mod structs;

#[tokio::main]
async fn main() {
    println!("Hello, which chain is your wallet from?");
    println!("1. Ethereum");
    println!("2. Solana");

    let mut choice = String::new();
    loop {
        stdin()
            .read_line(&mut choice)
            .expect("Did not enter a correct string");

        match choice.as_str() {
            "1\n" => {
                ethereum::main().await;
                break;
            }
            "2\n" => {
                println!("To be implemented");
                break;
            }
            _ => println!("Please insert a valid option"),
        }
    }
}
