pub mod display;
pub mod utils;
use crate::enums::Action;

pub async fn main() {
    println!("You have selected the Ethereum Chain");
    let wallet: String = display::get_wallet();

    loop {
        let action: Action = display::action_input();

        match action {
            Action::Balance => {
                display::display_balance(&wallet.as_str()).await.unwrap();
            }
            Action::Fiat => {
                display::display_fiat(&wallet.as_str()).await.unwrap();
            }
            Action::Transactions => {
                display::display_transactions(&wallet.as_str())
                    .await
                    .unwrap();
            }

            _ => {
                println!("Please input a valid option\n");
            }
        }
    }
}
