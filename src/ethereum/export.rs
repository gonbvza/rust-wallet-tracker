use std::fs::File;

use async_trait::async_trait;
use csv::Writer;

use crate::{
    errors::WalletError,
    ethereum::utils::{self, get_transactions},
    wallet_traits::WalletExport,
};

use super::ether_account::EtherAccount;

#[async_trait]
impl WalletExport for EtherAccount {
    /// Exports wallet statistics and transactions to two CSV files:
    /// - `statistics.csv`
    /// - `transactions.csv`
    async fn export_to_csv(&self) -> Result<(), WalletError> {
        let stats = utils::generate_statistics(&self.wallet).await?;
        let tx_count: i32 = stats.total_transactions.parse()?;
        let transactions = get_transactions(&self.wallet, tx_count).await?;

        let stats_file = File::create("statistics.csv")?;
        let mut stats_writer = Writer::from_writer(stats_file);

        stats_writer.write_record(&[
            "Address",
            "Total Transactions",
            "Average Gas",
            "Average ETH",
            "First Transaction",
        ])?;

        stats_writer.write_record(&[
            stats.address,
            stats.total_transactions,
            stats.average_gas.to_string(),
            stats.average_eth.to_string(),
            stats.first_transaction,
        ])?;

        stats_writer.flush()?;

        let tx_file = File::create("transactions.csv")?;
        let mut tx_writer = Writer::from_writer(tx_file);

        tx_writer.write_record(&["From", "To", "Gas", "Quantity", "Date"])?;

        for tx in transactions {
            tx_writer.write_record(&[tx.from, tx.to, tx.gas, tx.quantity.to_string(), tx.date])?;
        }

        tx_writer.flush()?;

        println!(
            "Exported statistics and transactions for {} to CSV files",
            &self.wallet
        );
        Ok(())
    }
}
