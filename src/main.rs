mod learning;
use learning::wallet::WalletTracker;
use learning::primitives::Primitives;

fn main() {
    println!("Bitcoin Wallet Balance Tracker");

    let mut my_wallet = WalletTracker::new(
        "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa", 0.5, false
    );

    my_wallet.display_info();
    my_wallet.receive(0.25);
    my_wallet.display_info();
    match my_wallet.send(0.1, 0.0005) {
        Ok(()) => println!("Transaction successful!"),
        Err(e) => println!("Transaction failed: {e}"),
    }
    match my_wallet.send(1.0, 0.0001) {
        Ok(()) => println!("Transaction successful!"),
        Err(e) => println!("Transaction failed: {e}"),
    }
    my_wallet.display_info();
    let mut test_wallet = WalletTracker::new(
        "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx",
        1.0,
        true
    );
    test_wallet.receive(0.5);
    test_wallet.display_info();

    Primitives::explore_integers();
}
