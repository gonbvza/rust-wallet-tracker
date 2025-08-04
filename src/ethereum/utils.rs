use crate::{date_utils, structs::Transaction};
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
            "&offset=10",
            "&sort=asc",
            "&apikey={}"
        ),
        wallet, etherscan_token
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
