use async_trait::async_trait;

use super::ether_account::EtherAccount;
use super::utils::{self, get_average_gas, get_transactions};
use crate::errors::WalletError;
use crate::ethereum::utils::{get_balance, get_fiat_balance};
use crate::structs::Transaction;
use crate::{input, wallet_traits};

#[async_trait]
impl wallet_traits::WalletDisplay for EtherAccount {
    /// Displays the wallet's ETH balance in the console.
    ///
    /// # Arguments
    /// * `wallet` - Wallet address as a string slice.
    ///
    /// # Errors
    /// Returns an error if balance retrieval fails.
    async fn display_balance(&self) -> Result<(), WalletError> {
        match get_balance(&self.wallet).await {
            Ok(value) => {
                println!("The balance of the wallet in ether is: {}", value);
                Ok(())
            }
            Err(error) => {
                error.display_error();
                Err(error)
            }
        }
    }

    /// Displays the wallet's fiat (USD) balance in the console.
    ///
    /// Uses Coinbase exchange rates for conversion.
    ///
    /// # Arguments
    /// * `wallet` - Wallet address as a string slice.
    async fn display_fiat(&self) -> Result<(), WalletError> {
        let usd_balanace = get_fiat_balance(&self.wallet).await?;
        println!("Balance in USD is: {}$\n", usd_balanace);

        Ok(())
    }

    /// Displays a list of the wallet's transactions in detailed format.
    ///
    /// The user is prompted for how many transactions to display.
    ///
    /// # Arguments
    /// * `wallet` - Wallet address as a string slice.
    async fn display_transactions(&self) -> Result<(), WalletError> {
        let transactions_offset = input::get_transaction_offset();
        let transactions: Vec<Transaction> =
            get_transactions(&self.wallet, transactions_offset).await?;
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
    async fn display_average_gas(&self) -> Result<(), WalletError> {
        let transactions_offset = input::get_transaction_offset();
        let average_gas = get_average_gas(&self.wallet, transactions_offset).await?;
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
    async fn display_statistics(&self) -> Result<(), WalletError> {
        let stats = utils::generate_statistics(&self.wallet).await?;
        print!("{}", stats);
        Ok(())
    }
}
