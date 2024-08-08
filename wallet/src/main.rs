use dotenv::from_filename;
use std::env;

use bdk::electrum_client::Client;
use bdk::{
    bitcoin::Network, blockchain::ElectrumBlockchain, database::MemoryDatabase, SyncOptions, Wallet,
};

fn main() {
    println!("Initiate wallet!....");
    from_filename(".env").ok();

    let descriptor = env::var("WALLET_DESCRIPTOR").unwrap();
    println!("Descriptor: {}", descriptor);

    let wallet = Wallet::new(
        descriptor.into(),
        None,
        Network::Testnet,
        MemoryDatabase::default(),
    )
    .expect("Failed to create wallet");

    let blockchain =
        ElectrumBlockchain::from(Client::new("ssl://electrum.blockstream.info:6002").unwrap());
    wallet
        .sync(&blockchain, SyncOptions::default())
        .expect("Failed to sync wallet");

    let balance = wallet.get_balance().expect("Failed to get balance");
    dbg!(balance);

    let address = wallet.get_address(AddressIndex::New).expect("failed to get address index!");
    dbg!(address);

}
