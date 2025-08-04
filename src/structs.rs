#[derive(Debug)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub quantity: f64,
    pub gas: String,
    pub date: String,
}

impl Transaction {
    pub fn display(&self) {
        println!("Transaction was the {}", self.date);
        println!("From: {}", self.from);
        println!("To: {}", self.to);
        println!("Gas used: {}", self.gas);
        println!("Value: {} ether", self.quantity);
        println!("")
    }
}
