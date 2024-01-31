use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;

const MAX_ITER : i32 = 10; //number of blocks to get before terminating

async fn listen_to_blocks() {
    let rpc_url = "https://api.testnet.solana.com".to_string(); // Using testnet for testing, tokens have no real-world value, no limitations. will likely swap to mainnet once tested and working.
    let rpc_client = RpcClient::new(rpc_url); //connect to rpc endpoint
    let mut iter = 0;

    loop{
        let recent_block = rpc_client.get_latest_blockhash_with_commitment(CommitmentConfig::confirmed()); //get most recent confirmed block
        match recent_block {
            Ok((blockhash, block_height)) => {
                //print information to the console if values exist
                println!("Block Height: {}", block_height);
                println!("Blockhash: {:?}", blockhash);
            }
            Err(err) => {
                //handle error
                eprintln!("Error getting recent block hash: {}", err);  
            }
        }

        //check iteration and wait 5 seconds
        iter += 1;
        if iter >= MAX_ITER {break;}
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}

#[tokio::main]
async fn main() {
    listen_to_blocks().await;
}
