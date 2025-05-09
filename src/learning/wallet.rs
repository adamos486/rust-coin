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

    pub fn send(&mut self, amount: f64, fee: f64) -> Result<(), String> {
        let total_amount = amount + fee;
        if total_amount > self.balance_btc {
            return Err(format!("Insufficient funds: have {}, need {}",
                self.balance_btc, total_amount));
        }

        self.balance_btc -= total_amount;
        self.num_transactions += 1;
        println!("Sent {} BTC with fee {}. New balance: {}", amount, fee,
            self.balance_btc);
        Ok(())
    }

    pub fn balance_in_satoshis(&self) -> u64 {
        const SATOSHI_PER_BTC: u64 = 100_000_000;
        (self.balance_btc * SATOSHI_PER_BTC as f64) as u64
    }

    pub fn display_info(&self) {
        println!("Wallet Information:");
        println!("-------------------");
        println!("Address: {}", self.address);
        println!("Balance: {} BTC ({} satoshis)", 
                self.balance_btc, self.balance_in_satoshis());
        println!("Network: {}", if self.is_testnet { "Testnet" } else { "Mainnet" });
        println!("Transaction count: {}", self.num_transactions);
    }
}