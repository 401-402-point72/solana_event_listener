mod solana;
use std::io;

#[tokio::main]
async fn main() {
    println!("Point72 Blockchain Menu: \n(1) listener\n");

    solana::listen_to_slots().await;

}