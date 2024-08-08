use dotenv::from_filename;
use std::env;

use bdk::{bitcoin::Network, Wallet};

fn main() {
    println!("Initiate wallet!....");
    from_filename(".env").ok();

    let descriptor = env::var("WALLET_DESCRIPTOR").unwrap();
    println!("Descriptor: {}", descriptor);
    // dbg!(descriptor);

    let wallet = Wallet::new(
        descriptor.into(),
        None,
        Network::Testnet,
        MemoryDatabase::default(),
    )?;
    dbg!(wallet);
}
