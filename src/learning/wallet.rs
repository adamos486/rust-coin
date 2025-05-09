pub struct WalletTracker {
    pub address: String,
    pub balance_btc: f64,
    pub is_testnet: bool,
    pub num_transactions: u32,
}

impl WalletTracker {
    pub fn new(address: &str, initial_balance: f64, is_testnet: bool) -> Self {
        WalletTracker {
            address: address.to_string(),
            balance_btc: initial_balance,
            is_testnet,
            num_transactions: 0,
        }
    }

    pub fn receive(&mut self, amount: f64) {
        self.balance_btc += amount;
        self.num_transactions += 1;
        println!("Received {} BTC. New balance: {}", amount, self.balance_btc);
    }
}