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

    loop {
        stdin()
            .read_line(&mut wallet)
            .expect("Please input a valid string");

        // Remove trailing newline
        wallet.truncate(wallet.len() - 1);

        if !wallet.starts_with("0x") || wallet.len() != 42 {
            println!("Please input a valid address\n");
            println!("Input your wallet again");
            wallet.clear();
            continue;
        }

        return wallet;
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
