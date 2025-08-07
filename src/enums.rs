/// Represents the different user actions available in the wallet tracker CLI.
#[derive(Debug)]
pub enum Action {
    /// Display the wallet's balance in Ether.
    Balance,
    /// Display the wallet's balance in fiat currency (e.g., USD).
    Fiat,
    /// Show recent transactions for the wallet.
    Transactions,
    /// Display calculated statistics for the wallet.
    Stats,
    /// Export wallet data to a file.
    Export,
    /// Show the average gas used in recent transactions.
    Gas,
    /// Exit the application.
    Exit,
}
