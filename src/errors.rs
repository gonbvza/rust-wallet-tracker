use csv;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WalletError {
    #[error("Network request failed: {0}")]
    Network(#[from] reqwest::Error),

    #[error("JSON parse error: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("Integer parse error: {0}")]
    IntParse(#[from] std::num::ParseIntError),

    #[error("Integer parse error: {0}")]
    FloatParse(#[from] std::num::ParseFloatError),

    #[error("Invalid wallet address: {address}")]
    InvalidAddress { address: String },

    #[error("Missing field: {field}")]
    Missing { field: String },

    #[error("No transactions for this address")]
    NoTransactions,

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
}
