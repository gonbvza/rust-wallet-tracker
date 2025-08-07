use crate::errors::WalletError;
use async_trait::async_trait;

/// Defines display operations for wallet-related data such as balance, fiat value, transactions, and statistics.
#[async_trait]
pub trait WalletDisplay {
    async fn display_balance(&self) -> Result<(), WalletError>;
    async fn display_fiat(&self) -> Result<(), WalletError>;
    async fn display_transactions(&self) -> Result<(), WalletError>;
    async fn display_average_gas(&self) -> Result<(), WalletError>;
    async fn display_statistics(&self) -> Result<(), WalletError>;
}

/// Defines export operations for wallet-related data.
#[async_trait]
pub trait WalletExport {
    async fn export_to_csv(&self) -> Result<(), WalletError>;
}
