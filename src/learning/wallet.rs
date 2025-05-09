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

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_precision_loss)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_wallet() {
        let address = "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa";
        let initial_balance = 1.5;
        let is_testnet = false;

        let wallet = WalletTracker::new(address, initial_balance, is_testnet);

        assert_eq!(wallet.address, address);
        assert_eq!(wallet.balance_btc, initial_balance);
        assert_eq!(wallet.is_testnet, is_testnet);
        assert_eq!(wallet.num_transactions, 0);
    }

    #[test]
    fn test_receive() {
        let mut wallet = WalletTracker::new("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa", 1.0, false);
        let initial_balance = wallet.balance_btc;
        let initial_transactions = wallet.num_transactions;

        wallet.receive(0.5);

        assert_eq!(wallet.balance_btc, initial_balance + 0.5);
        assert_eq!(wallet.num_transactions, initial_transactions + 1);
    }

    #[test]
    fn test_send_success() {
        let mut wallet = WalletTracker::new("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa", 1.0, false);
        let initial_balance = wallet.balance_btc;
        let initial_transactions = wallet.num_transactions;
        let amount = 0.5;
        let fee = 0.0001;

        let result = wallet.send(amount, fee);

        assert!(result.is_ok());
        assert_eq!(wallet.balance_btc, initial_balance - amount - fee);
        assert_eq!(wallet.num_transactions, initial_transactions + 1);
    }

    #[test]
    fn test_send_insufficient_funds() {
        let mut wallet = WalletTracker::new("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa", 0.5, false);
        let initial_balance = wallet.balance_btc;
        let initial_transactions = wallet.num_transactions;
        let amount = 1.0;
        let fee = 0.0001;

        let result = wallet.send(amount, fee);

        assert!(result.is_err());
        assert_eq!(wallet.balance_btc, initial_balance); // Balance should remain unchanged
        assert_eq!(wallet.num_transactions, initial_transactions); // Transaction count should remain unchanged

        if let Err(error_msg) = result {
            assert!(error_msg.contains("Insufficient funds"));
        }
    }

    #[test]
    fn test_balance_in_satoshis() {
        let wallet = WalletTracker::new("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa", 1.23456789, false);

        // 1 BTC = 100,000,000 satoshis
        // Due to floating point precision, we need to check the actual value
        let actual_satoshis = wallet.balance_in_satoshis();
        // The value should be very close to 123,456,789
        assert!(actual_satoshis >= 123_456_788 && actual_satoshis <= 123_456_789);

        // Test with zero balance
        let zero_wallet = WalletTracker::new("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa", 0.0, false);
        assert_eq!(zero_wallet.balance_in_satoshis(), 0);
    }

    #[test]
    fn test_edge_cases() {
        // Test with zero initial balance
        let mut zero_wallet = WalletTracker::new("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa", 0.0, false);
        assert_eq!(zero_wallet.balance_btc, 0.0);

        // Test receiving to zero balance
        zero_wallet.receive(0.1);
        assert_eq!(zero_wallet.balance_btc, 0.1);

        // Test with very small amounts
        let small_wallet = WalletTracker::new("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa", 0.00000001, false);
        assert_eq!(small_wallet.balance_in_satoshis(), 1); // 1 satoshi

        // Test with very large balance
        let large_wallet = WalletTracker::new("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa", 21_000_000.0, false);
        assert_eq!(large_wallet.balance_in_satoshis(), 2_100_000_000_000_000);
    }

    #[test]
    fn test_testnet_flag() {
        let mainnet_wallet = WalletTracker::new("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa", 1.0, false);
        assert!(!mainnet_wallet.is_testnet);

        let testnet_wallet = WalletTracker::new("tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx", 1.0, true);
        assert!(testnet_wallet.is_testnet);
    }

    #[test]
    fn test_multiple_transactions() {
        let mut wallet = WalletTracker::new("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa", 5.0, false);
        assert_eq!(wallet.num_transactions, 0);

        // Perform multiple transactions
        wallet.receive(1.0);
        assert_eq!(wallet.num_transactions, 1);

        wallet.receive(0.5);
        assert_eq!(wallet.num_transactions, 2);

        let _ = wallet.send(0.1, 0.0001);
        assert_eq!(wallet.num_transactions, 3);

        let _ = wallet.send(0.2, 0.0001);
        assert_eq!(wallet.num_transactions, 4);

        // Due to floating point precision, we need to check with a small epsilon
        let expected_balance = 5.0 + 1.0 + 0.5 - 0.1 - 0.0001 - 0.2 - 0.0001;
        assert!((wallet.balance_btc - expected_balance).abs() < 0.000000001);
    }

    #[test]
    fn test_floating_point_precision() {
        let mut wallet = WalletTracker::new("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa", 1.0, false);

        // Add a very small amount that might cause floating point precision issues
        wallet.receive(0.00000001); // 1 satoshi

        // The balance should be exactly 1.00000001 BTC
        assert!((wallet.balance_btc - 1.00000001).abs() < f64::EPSILON);

        // Check satoshi conversion is correct
        assert_eq!(wallet.balance_in_satoshis(), 100_000_001);
    }

    #[test]
    fn test_display_info() {
        // This test doesn't actually check the output, since display_info only prints to stdout
        // But we can at least verify it doesn't panic
        let wallet = WalletTracker::new("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa", 1.5, false);

        // This should not panic
        wallet.display_info();

        // Create a testnet wallet and display its info
        let testnet_wallet = WalletTracker::new("tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx", 2.5, true);
        testnet_wallet.display_info();

        // Create a wallet with zero balance
        let zero_wallet = WalletTracker::new("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa", 0.0, false);
        zero_wallet.display_info();
    }
}