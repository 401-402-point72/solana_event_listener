use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
//use chrono::{DateTime, Utc, TimeZone};

const MAX_ITER : i32 = 10; //number of slots to get and try to retrieve a block from before terminating
const MAX_RETRIES : i32 = 5; //number of retries to retrieve block from the current slot before timing out

async fn listen_to_blocks() {
    let rpc_url = "https://api.devnet.solana.com".to_string(); // Using devnet for testing, will likely swap to mainnet once tested and working.
    let rpc_client = RpcClient::new(rpc_url); //connect to rpc endpoint
    let mut iter = 0;

    loop{
        let mut retries = 0;
        let latest_slot = rpc_client.get_slot_with_commitment(CommitmentConfig::finalized()); //get latest slot
        match latest_slot{
            Ok(slot) => {
                println!("\nLatest Slot: {}", slot);

                let latest_block = rpc_client.get_block(slot); //get block at latest slot
                loop{
                    match latest_block{
                        Ok(_encoded_confirmed_block) =>{
                            //println!("Latest Block: {:?}", encoded_confirmed_block);
                            println!("Block at slot {} found and information retrieved",slot);
                            break;
                        }
                        Err(ref err) =>{
                            retries += 1;
                            if retries >= MAX_RETRIES {
                                eprintln!("MAX_RETRIES reached, timing out and moving to next slot");
                                break;
                            }
                            eprintln!("Error getting latest block: {}", err);
                            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                        }
                    }
                }
            }
            Err(err) => {
                eprintln!("Error getting latest slot: {}", err);
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
