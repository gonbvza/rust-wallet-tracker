use std::fs::File;

use csv::Writer;
use dotenv::dotenv;
use reqwest::Client;
use serde_json::{json, Value};

use crate::{
    date_utils,
    errors::WalletError,
    structs::{Statistics, Transaction},
};

/// Number of Wei in one Ether.
const WEI_VALUE: i64 = 1_000_000_000_000_000_000;

/// Returns the ETH balance of the given wallet address.
///
/// Uses a JSON-RPC call to an Ethereum node.
///
/// # Arguments
/// * `wallet` - A string slice containing the wallet address.
///
/// # Returns
/// * `Ok(f64)` - The balance in ETH.
/// * `Err` - If the API call fails or parsing fails.
pub async fn get_balance(wallet: &str) -> Result<f64, WalletError> {
    if !wallet.starts_with("0x") || wallet.len() != 42 {
        return Err(WalletError::InvalidAddress {
            address: wallet.to_string(),
        });
    }

    let client = Client::new();

    let body = json!({
        "id": 1,
        "jsonrpc": "2.0",
        "method": "eth_getBalance",
        "params": [wallet, "pending"]
    });

    let response = client
        .post("https://nd-422-757-666.p2pify.com/0a9d79d93fb2f4a4b1e04695da2b77a7")
        .json(&body)
        .send()
        .await?
        .json::<Value>()
        .await?;

    let hex_balance = response["result"].as_str().ok_or(WalletError::Missing {
        field: "result".to_string(),
    })?;

    let wei_balance = i64::from_str_radix(&hex_balance[2..], 16)?;
    Ok(wei_balance as f64 / WEI_VALUE as f64)
}

/// Returns the fiat balance (in USD) of a given wallet address.
///
/// Fetches ETH balance and converts it to USD using Coinbase exchange rates.
///
/// # Arguments
/// * `wallet` - A string slice containing the wallet address.
pub async fn get_fiat_balance(wallet: &str) -> Result<f64, WalletError> {
    let eth_balance = get_balance(wallet).await?;
    let client = Client::new();

    let response = client
        .get("https://api.coinbase.com/v2/exchange-rates?currency=ETH")
        .send()
        .await?
        .json::<Value>()
        .await?;

    let rate = response["data"]["rates"]["USD"]
        .as_str()
        .ok_or(WalletError::Missing {
            field: "data.rates.USD".to_string(),
        })?;

    Ok(rate.parse::<f64>()? * eth_balance)
}

/// Returns a list of transactions for the given wallet.
///
/// Uses the Etherscan API.
///
/// # Arguments
/// * `wallet` - Wallet address as a string slice.
/// * `transactions_offset` - Max number of transactions to fetch.
pub async fn get_transactions(
    wallet: &str,
    transactions_offset: i32,
) -> Result<Vec<Transaction>, WalletError> {
    dotenv().ok();
    let client = Client::new();

    let etherscan_token = std::env::var("ETHERSCAN_KEY").expect("Etherscan token must be set.");

    let url = format!(
        concat!(
            "https://api.etherscan.io/v2/api?",
            "chainid=1",
            "&module=account",
            "&action=txlist",
            "&address={}",
            "&startblock=0",
            "&endblock=99999999",
            "&page=1",
            "&offset={}",
            "&sort=asc",
            "&apikey={}"
        ),
        wallet, transactions_offset, etherscan_token
    );

    let response = client.get(url).send().await?.json::<Value>().await?;
    let transactions = response["result"].as_array().ok_or(WalletError::Missing {
        field: "result".to_string(),
    })?;

    let mut parsed_transactions = Vec::new();

    for tx in transactions {
        let parsed_tx = Transaction {
            from: tx["from"]
                .as_str()
                .ok_or(WalletError::Missing {
                    field: "from".to_string(),
                })?
                .to_string(),
            to: tx["to"]
                .as_str()
                .ok_or(WalletError::Missing {
                    field: "to".to_string(),
                })?
                .to_string(),
            gas: tx["gasUsed"]
                .as_str()
                .ok_or(WalletError::Missing {
                    field: "gasUsed".to_string(),
                })?
                .to_string(),
            quantity: tx["value"]
                .as_str()
                .ok_or(WalletError::Missing {
                    field: "value".to_string(),
                })?
                .parse::<f64>()?
                / WEI_VALUE as f64,
            date: date_utils::epoch_converter(
                tx["timeStamp"]
                    .as_str()
                    .ok_or(WalletError::Missing {
                        field: "timeStamp".to_string(),
                    })?
                    .to_string(),
            ),
        };

        parsed_transactions.push(parsed_tx);
    }

    Ok(parsed_transactions)
}

/// Returns the average gas used across transactions of a wallet.
///
/// # Arguments
/// * `wallet` - Wallet address.
/// * `transactions_offset` - Number of transactions to include.
pub async fn get_average_gas(wallet: &str, transactions_offset: i32) -> Result<f64, WalletError> {
    let transactions = get_transactions(wallet, transactions_offset).await?;

    let total_gas: i32 = transactions
        .iter()
        .map(|tx| tx.gas.parse::<i32>().unwrap_or(0))
        .sum();

    Ok(total_gas as f64 / transactions.len() as f64)
}

/// Returns the average ETH transferred in transactions of a wallet.
pub async fn get_average_eth(wallet: &str, transactions_offset: i32) -> Result<f64, WalletError> {
    let transactions = get_transactions(wallet, transactions_offset).await?;

    let total_eth: f64 = transactions.iter().map(|tx| tx.quantity).sum();
    Ok(total_eth / transactions.len() as f64)
}

/// Generates wallet statistics (transaction count, averages, first activity).
pub async fn generate_statistics(wallet: &str) -> Result<Statistics, WalletError> {
    let tx_count = get_total_transactions(wallet).await?;
    let average_gas = get_average_gas(wallet, (tx_count + 1) as i32).await?;
    let average_eth = get_average_eth(wallet, (tx_count + 1) as i32).await?;
    let first_transaction = get_first_transaction_date(wallet).await?;

    Ok(Statistics {
        address: wallet.to_string(),
        total_transactions: tx_count.to_string(),
        average_gas,
        average_eth,
        first_transaction,
    })
}

/// Returns the total number of transactions for a wallet.
pub async fn get_total_transactions(wallet: &str) -> Result<i64, WalletError> {
    let client = Client::new();
    let url = format!("https://api.blockcypher.com/v1/eth/main/addrs/{}", wallet);
    let response = client.get(url).send().await?.json::<Value>().await?;

    response["n_tx"].as_i64().ok_or(WalletError::Missing {
        field: "n_tx".to_string(),
    })
}

/// Returns the date of the first transaction of a wallet.
///
/// The date is returned in `YYYY-MM-DD` format.
pub async fn get_first_transaction_date(wallet: &str) -> Result<String, WalletError> {
    let transactions = get_transactions(wallet, 1).await?;

    match transactions.first() {
        Some(first_tx) => {
            let date_str = first_tx.date.split(' ').next().unwrap_or("");
            Ok(date_str.to_string())
        }
        None => Err(WalletError::NoTransactions),
    }
}

/// Exports wallet statistics and transactions to two CSV files:
/// - `statistics.csv`
/// - `transactions.csv`
pub async fn export_to_csv(wallet: &str) -> Result<(), WalletError> {
    let stats = generate_statistics(wallet).await?;
    let tx_count: i32 = stats.total_transactions.parse()?;
    let transactions = get_transactions(wallet, tx_count).await?;

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
        wallet
    );
    Ok(())
}
