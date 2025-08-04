use crate::{enums::Action, structs::Transaction};
use std::io::stdin;

use crate::ethereum::utils::{get_balance, get_fiat_balance};

use super::utils::get_transactions;

pub fn get_wallet() -> String {
    let mut wallet = String::new();
    println!("Please input your wallet");
    stdin()
        .read_line(&mut wallet)
        .expect("Please input a valid string");

    wallet.truncate(wallet.len() - 1);

    wallet
}

pub fn action_input() -> Action {
    println!("What do you want to do with this wallet");
    println!("1. Balance");
    println!("2. Fiat");
    println!("3. Transactions");
    println!("4. Average Gas");
    println!("5. Statistics");
    println!("6. Export");
    println!("7. Exit");

    let mut action = String::new();
    loop {
        stdin()
            .read_line(&mut action)
            .expect("Please type a valid string");

        match action.as_str() {
            "1\n" => return Action::Balance,
            "2\n" => return Action::Fiat,
            "3\n" => return Action::Transactions,
            "4\n" => return Action::Gas,
            "5\n" => return Action::Stats,
            "6\n" => return Action::Export,
            "7\n" => return Action::Exit,

            _ => println!("Please type a valid option"),
        }
    }
}

pub async fn display_balance(wallet: &str) -> Result<(), Box<dyn std::error::Error>> {
    let eth_balanace = get_balance(wallet).await?;
    println!("Balance in ether is: {}\n", eth_balanace);

    Ok(())
}

pub async fn display_fiat(wallet: &str) -> Result<(), Box<dyn std::error::Error>> {
    let usd_balanace = get_fiat_balance(wallet).await?;
    println!("Balance in USD is: {}$\n", usd_balanace);

    Ok(())
}

pub async fn display_transactions(wallet: &str) -> Result<(), Box<dyn std::error::Error>> {
    let transactions: Vec<Transaction> = get_transactions(wallet).await?;
    for tx in transactions {
        tx.display();
    }
    Ok(())
}
