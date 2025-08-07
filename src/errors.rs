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

impl WalletError {
    pub fn display_error(&self) {
        match self {
            WalletError::InvalidAddress { address } => {
                eprintln!("Invalid wallet address: {}", address);
                eprintln!("Addresses should start with 0x and be 42 characters long");
            }
            WalletError::Network(req_err) => {
                eprintln!("Network error: {}", req_err);
                eprintln!("Check your internet connection and try again");
            }
            WalletError::Missing { field } => {
                eprintln!("Missing field: {}", field);
                eprintln!("The API response format may have changed");
            }
            WalletError::JsonParse(err) => {
                eprintln!("JSON parse error: {}", err);
                eprintln!("The API response may not be valid JSON");
            }
            WalletError::IntParse(err) => {
                eprintln!("Integer parse error: {}", err);
                eprintln!("The data might contain non-numeric values where numbers are expected");
            }
            WalletError::FloatParse(err) => {
                eprintln!("Float parse error: {}", err);
                eprintln!("The data might contain invalid decimal values");
            }
            WalletError::NoTransactions => {
                eprintln!("No transactions found for this address");
                eprintln!("Try again later or verify the address has activity");
            }
            WalletError::Io(err) => {
                eprintln!("I/O error: {}", err);
                eprintln!("Check file paths, permissions, or disk availability");
            }
            WalletError::Csv(err) => {
                eprintln!("CSV error: {}", err);
                eprintln!("Ensure the CSV file is properly formatted");
            }
        }
    }
}
