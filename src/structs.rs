use std::fmt;

#[derive(Debug)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub quantity: f64,
    pub gas: String,
    pub date: String,
}

impl fmt::Display for Transaction {
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
    pub fn display_detailed(&self) {
        println!("{}", self);
    }

    pub fn display_compact(&self) {
        println!(
            "{} -> {} | {} ETH",
            &self.from[..8],
            &self.to[..8],
            self.quantity
        );
    }
}

#[derive(Debug)]
pub struct Statistics {
    pub address: String,
    pub total_transactions: String,
    pub average_gas: f64,
    pub average_eth: f64,
    pub first_transaction: String,
}

impl fmt::Display for Statistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Wallet adress {}\n\
             Total number of transactions: {}\n\
             Average gas per transaction: {:.2}\n\
             Average eth per transaction: {:}\n\
             Date of first transaction: {}\n\
             ",
            self.address,
            self.total_transactions,
            self.average_gas,
            self.average_eth,
            self.first_transaction,
        )
    }
}
