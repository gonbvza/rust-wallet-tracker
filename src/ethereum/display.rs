use super::utils::{self, get_average_gas, get_transactions};
use crate::errors::WalletError;
use crate::ethereum::utils::{get_balance, get_fiat_balance};
use crate::{enums::Action, structs::Transaction};
use std::io::stdin;

/// Maximum number of transactions that can be fetched/displayed at once.
const MAX_TRANSACTION_OFFSET: i32 = 20;

/// Prompts the user to input a wallet address via standard input.
///
/// # Returns
/// * A `String` containing the wallet address.
pub fn get_wallet() -> String {
    let mut wallet = String::new();
    println!("Please input your wallet");

    stdin()
        .read_line(&mut wallet)
        .expect("Please input a valid string");

    // Remove trailing newline
    wallet.truncate(wallet.len() - 1);

    wallet
}

/// Prompts the user to select an action from a list of wallet operations.
///
/// Displays a numbered menu and waits for valid input.
///
/// # Returns
/// * An [`Action`](crate::enums::Action) corresponding to the user's choice.
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

/// Prompts the user to specify how many transactions should be displayed.
///
/// Ensures the number is within the allowed range (`1..=MAX_TRANSACTION_OFFSET`).
///
/// # Returns
/// * An `i32` representing the number of transactions requested.
pub fn get_transaction_offset() -> i32 {
    loop {
        println!(
            "How many transactions do you want to see (Max {}): ",
            MAX_TRANSACTION_OFFSET
        );

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<i32>() {
            Ok(num) if num > 0 && num <= MAX_TRANSACTION_OFFSET => {
                return num;
            }
            _ => {
                println!(
                    "Please input a number between 1 and {}",
                    MAX_TRANSACTION_OFFSET
                );
            }
        }
    }
}

/// Displays the wallet's ETH balance in the console.
///
/// # Arguments
/// * `wallet` - Wallet address as a string slice.
///
/// # Errors
/// Returns an error if balance retrieval fails.
pub async fn display_balance(wallet: &str) -> Result<(), WalletError> {
    let eth_balanace = get_balance(wallet).await?;
    println!("Balance in ether is: {}\n", eth_balanace);

    Ok(())
}

/// Displays the wallet's fiat (USD) balance in the console.
///
/// Uses Coinbase exchange rates for conversion.
///
/// # Arguments
/// * `wallet` - Wallet address as a string slice.
pub async fn display_fiat(wallet: &str) -> Result<(), WalletError> {
    let usd_balanace = get_fiat_balance(wallet).await?;
    println!("Balance in USD is: {}$\n", usd_balanace);

    Ok(())
}

/// Displays a list of the wallet's transactions in detailed format.
///
/// The user is prompted for how many transactions to display.
///
/// # Arguments
/// * `wallet` - Wallet address as a string slice.
pub async fn display_transactions(wallet: &str) -> Result<(), WalletError> {
    let transactions_offset = get_transaction_offset();
    let transactions: Vec<Transaction> = get_transactions(wallet, transactions_offset).await?;
    for tx in transactions {
        tx.display_detailed();
    }
    Ok(())
}

/// Displays the average gas used in recent transactions for a wallet.
///
/// The number of transactions is determined by user input.
///
/// # Arguments
/// * `wallet` - Wallet address as a string slice.
pub async fn display_average_gas(wallet: &str) -> Result<(), WalletError> {
    let transactions_offset = get_transaction_offset();
    let average_gas = get_average_gas(wallet, transactions_offset).await?;
    println!(
        "The average gas for the last transactions was: {}\n",
        average_gas
    );
    Ok(())
}

/// Displays overall wallet statistics including transaction history details.
///
///
/// # Arguments
/// * `wallet` - Wallet address as a string slice.
pub async fn display_statistics(wallet: &str) -> Result<(), WalletError> {
    let stats = utils::generate_statistics(wallet).await?;
    print!("{}", stats);
    Ok(())
}
