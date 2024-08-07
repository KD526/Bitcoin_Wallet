use dotenv::from_filename;
use std::env;

fn main() {
    println!("Initiate wallet!....");
    dotenv::from_filename(".env").ok();

    let descriptor = env::var("WALLET_DESCRIPTOR").unwrap();
    println!("Descriptor: {}", descriptor);

}
