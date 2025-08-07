
use std::fmt;

/// Represents a single Ethereum transaction.
#[derive(Debug)]
pub struct Transaction {
    /// Sender wallet address.
    pub from: String,
    /// Recipient wallet address.
    pub to: String,
    /// Value transferred in ETH.
    pub quantity: f64,
    /// Gas used for the transaction (in wei).
    pub gas: String,
    /// Date of the transaction (as a string).
    pub date: String,
}

impl fmt::Display for Transaction {
    /// Formats the transaction details in a human-readable multiline format.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Transaction on {}\n\
             From: {}\n\
             To: {}\n\
             Value: {} ETH\n\
             Gas: {} wei\n",
            self.date, self.from, self.to, self.quantity, self.gas,
        )
    }
}

impl Transaction {
    /// Prints a detailed view of the transaction using `Display`.
    pub fn display_detailed(&self) {
        println!("{}", self);
    }

    /// Prints a compact, one-line summary of the transaction.
    pub fn display_compact(&self) {
        println!(
            "{} -> {} | {} ETH",
            &self.from[..8],
            &self.to[..8],
            self.quantity
        );
    }
}

/// Summary statistics for a wallet's transaction history.
#[derive(Debug)]
pub struct Statistics {
    /// Wallet address being analyzed.
    pub address: String,
    /// Total number of transactions.
    pub total_transactions: String,
    /// Average gas used per transaction.
    pub average_gas: f64,
    /// Average ETH transferred per transaction.
    pub average_eth: f64,
    /// Date of the wallet’s first transaction.
    pub first_transaction: String,
}

impl fmt::Display for Statistics {
    /// Formats the wallet statistics in a readable format for display.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Wallet address: {}\n\
             Total number of transactions: {}\n\
             Average gas per transaction: {:.2}\n\
             Average ETH per transaction: {}\n\
             Date of first transaction: {}\n",
            self.address,
            self.total_transactions,
            self.average_gas,
            self.average_eth,
            self.first_transaction,
        )
    }
}

