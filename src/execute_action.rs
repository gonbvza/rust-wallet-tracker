use std::io::{stdin, stdout, Write};

use crate::{
    enums::Action,
    errors::WalletError,
    wallet_traits::{WalletDisplay, WalletExport},
};

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

    stdout().flush().unwrap();

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
            _ => {
                println!("Please type a valid option");
                action.clear();
            }
        }
    }
}

/// Executes corresponding action based on user input
pub async fn execute_action<T: WalletDisplay + WalletExport>(
    account: T,
) -> Result<(), WalletError> {
    loop {
        let action: Action = action_input();

        match action {
            Action::Balance => {
                account.display_balance().await?;
            }
            Action::Fiat => {
                account.display_fiat().await?;
            }
            Action::Transactions => {
                account.display_transactions().await?;
            }
            Action::Gas => {
                account.display_average_gas().await?;
            }
            Action::Stats => {
                account.display_statistics().await?;
            }
            Action::Export => {
                account.export_to_csv().await?;
            }
            Action::Exit => {
                std::process::exit(0);
            }
        }
    }
}
