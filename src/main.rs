use std::error::Error;

use ethereum::ether_account::EtherAccount;
use execute_action::execute_action;
use tokio;

pub mod date_utils;
pub mod enums;
pub mod errors;
pub mod ethereum;
pub mod execute_action;
pub mod input;
pub mod structs;
pub mod wallet_traits;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let wallet: String = input::get_wallet();

    let ether_account = EtherAccount { wallet };
    execute_action(ether_account).await?;

    Ok(())
}
