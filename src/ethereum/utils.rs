use csv::Writer;
use std::fs::File;

use crate::{
    date_utils,
    structs::{Statistics, Transaction},
};
use dotenv::dotenv;
use reqwest::Client;
use serde_json::{json, Value};

const WEI_VALUE: i64 = 1000000000000000000;

pub async fn get_balance(wallet: &str) -> Result<f64, Box<dyn std::error::Error>> {
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

    let hex_balance = response["result"].as_str().ok_or("Missing balance field")?;
    let wei_balance: i64 = i64::from_str_radix(&hex_balance[2..], 16)?;
    let eth_balance: f64 = wei_balance as f64 / WEI_VALUE as f64;

    Ok(eth_balance)
}

pub async fn get_fiat_balance(wallet: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let eth_balance = get_balance(&wallet).await?;
    let client = Client::new();

    let response = client
        .get("https://api.coinbase.com/v2/exchange-rates?currency=ETH")
        .send()
        .await?
        .json::<Value>()
        .await?;

    let rate = response["data"]["rates"]["USD"]
        .as_str()
        .ok_or("Missing balance field")?;

    let value_usd = rate.parse::<f64>().unwrap() * eth_balance;

    Ok(value_usd)
}

pub async fn get_transactions(
    wallet: &str,
    transactions_offset: i32,
) -> Result<Vec<Transaction>, Box<dyn std::error::Error>> {
    let client = Client::new();
    dotenv().ok();

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

    let transactions = response["result"].as_array().ok_or("Missing field")?;

    let mut parsed_transactions: Vec<Transaction> = Vec::new();

    for tx in transactions {
        let parsed_tx = Transaction {
            from: tx["from"].as_str().ok_or("No field from")?.to_string(),
            to: tx["to"].as_str().ok_or("No field to")?.to_string(),
            gas: tx["gasUsed"]
                .as_str()
                .ok_or("No field gasUsed")?
                .to_string(),
            quantity: tx["value"]
                .as_str()
                .ok_or("No field value")?
                .to_string()
                .parse::<f64>()
                .unwrap()
                / WEI_VALUE as f64,
            date: date_utils::epoch_converter(
                tx["timeStamp"]
                    .as_str()
                    .ok_or("No field timeStamp")?
                    .to_string(),
            ),
        };

        parsed_transactions.push(parsed_tx);
    }

    Ok(parsed_transactions)
}

pub async fn get_average_gas(
    wallet: &str,
    transactions_offset: i32,
) -> Result<f64, Box<dyn std::error::Error>> {
    let transactions = get_transactions(wallet, transactions_offset).await?;
    let mut total_gas: i32 = 0;
    for tx in &transactions {
        total_gas += tx.gas.parse::<i32>().unwrap();
    }
    // f32 as the size of the vector will never go beyond 20
    Ok(total_gas as f64 / transactions.len() as f64)
}

pub async fn get_average_eth(
    wallet: &str,
    transactions_offset: i32,
) -> Result<f64, Box<dyn std::error::Error>> {
    let transactions = get_transactions(wallet, transactions_offset).await?;
    let mut total_eth: f64 = 0.0;
    for tx in &transactions {
        total_eth += tx.quantity;
    }
    // f32 as the size of the vector will never go beyond 20
    Ok(total_eth as f64 / transactions.len() as f64)
}

pub async fn generate_statistics(wallet: &str) -> Result<Statistics, Box<dyn std::error::Error>> {
    let tx_count = get_total_transactions(wallet).await?;
    let average_gas = get_average_gas(wallet, (tx_count + 1) as i32).await?;
    let average_eth = get_average_eth(wallet, (tx_count + 1) as i32).await?;
    let first_transaction = get_first_transaction_date(wallet).await?;
    let stats = Statistics {
        address: wallet.to_string(),
        total_transactions: tx_count.to_string(),
        average_gas,
        average_eth,
        first_transaction,
    };

    Ok(stats)
}

pub async fn get_total_transactions(wallet: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("https://api.blockcypher.com/v1/eth/main/addrs/{}", wallet);
    let response = client.get(url).send().await?.json::<Value>().await?;
    let tx_count = response["n_tx"].as_i64().ok_or("No field value")?;
    Ok(tx_count)
}

pub async fn get_first_transaction_date(
    wallet: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let transactions = get_transactions(wallet, 1).await?;
    match transactions.first() {
        Some(first_tx) => {
            let date_str = first_tx.date.split(' ').next().unwrap_or("");
            Ok(date_str.to_string())
        }
        None => Err("No transactions found".into()),
    }
}

pub async fn export_to_csv(wallet: &str) -> Result<(), Box<dyn std::error::Error>> {
    let stats = generate_statistics(wallet).await?;
    let tx_count: i32 = stats.total_transactions.parse()?;
    let transactions = get_transactions(wallet, tx_count).await?;

    let stats_file = File::create(format!("statistics.csv"))?;
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

    let tx_file = File::create(format!("transactions.csv"))?;
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
