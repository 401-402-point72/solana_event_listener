use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcBlockConfig;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_transaction_status::UiConfirmedBlock;
use solana_transaction_status::UiTransactionEncoding;
use solana_transaction_status::TransactionDetails;
use chrono::{LocalResult, Utc, TimeZone};

const MAX_ITER : i32 = 10; //number of slots to get and try to retrieve a block from before terminating
const MAX_RETRIES : i32 = 5; //number of retries to retrieve block from the current slot before timing out

// fn _parse_transactions(){ //parses through block transaction info

// }

// fn _parse_rewards(){ //parses through block rewards info

// }

use solana_transaction_status::{EncodedTransaction, EncodedTransactionWithStatusMeta, UiTransaction, EncodedConfirmedBlock, Reward};
use serde_json::json;


/* fn _parse_transactions(transactions: Vec<EncodedTransactionWithStatusMeta>) {
    for transaction in transactions {
        // Extract relevant transaction information and store it in the desired format or database
        let (signatures, message) = match &transaction.transaction {
            EncodedTransaction::Json(ui_transaction) => {
                if let Some(ui_transaction) = &ui_transaction.transaction {
                    (Some(&ui_transaction.signatures), Some(&ui_transaction.message))
                } else {
                    (None, None)
                }
            }
            _ => (None, None), // Handle other variants if needed
        };

        // Formulate transaction info JSON
        let transaction_info = match (signatures, message) {
            (Some(signatures), Some(message)) => json!({
                "signatures": signatures,
                "message": message,
                // Add more fields as needed
            }),
            _ => json!({}), // Handle the case where either signatures or message is None
        };

        // Store transaction_info or process it further
        println!("Parsed Transaction: {}", transaction_info);
    }
} */




fn _parse_rewards(rewards: Vec<Reward>) {
    for reward in rewards {
        // Extract relevant reward information and store it in the desired format or database
        let reward_info = json!({
            "pubkey": reward.pubkey,
            "lamports": reward.lamports,
            // Add more fields as needed
        });

        // Store reward_info or process it further
        println!("Parsed Reward: {}", reward_info);
    }
}







fn parse_block(encoded_confirmed_block: UiConfirmedBlock){ //parses info in block, prints to console (will store in database eventually)
    println!("Blockhash: {}", encoded_confirmed_block.blockhash);
    println!("Previous Blockhash: {}", encoded_confirmed_block.previous_blockhash);
    println!("Parent Slot: {}", encoded_confirmed_block.parent_slot);
    match encoded_confirmed_block.block_height {
        Some(height) => println!("Block Height: {}", height),
        None => println!("Block Height: N/A"),
    }
    match encoded_confirmed_block.block_time {
        Some(time) => {
            match Utc.timestamp_opt(time as i64, 0) { //convert to day-date-month-year-time format
                LocalResult::Single(datetime) => println!("Block Time: {}", datetime.to_rfc2822()),
                LocalResult::None => println!("Failed to parse block time"),
                LocalResult::Ambiguous(_, _) => println!("Block Time: Ambiguous"),
            }
        },
        None => println!("Block Time: N/A"),
    }
    //println!("Transactions: {:?}", encoded_confirmed_block.transactions);
    //println!("Rewards: {:?}", encoded_confirmed_block.rewards);

    /*if let Some(transactions) = encoded_confirmed_block.transactions{
        _parse_transactions(transactions);
    }
    else{
        println!("No transactions in this block.");
    }*/

    if let Some(rewards) = encoded_confirmed_block.rewards{
        _parse_rewards(rewards);
    }
    else{
        println!("No rewards in this block.");
    }
}

async fn listen_to_slots() {
    let rpc_url = "https://api.devnet.solana.com".to_string(); // Using devnet for testing, will likely swap to mainnet once tested and working.
    let rpc_client = RpcClient::new(rpc_url); //connect to rpc endpoint
    let mut iter = 0;
    let config = RpcBlockConfig {
        encoding: Some(UiTransactionEncoding::Base58),
        transaction_details: Some(TransactionDetails::None),
        rewards: Some(true),
        commitment: Some(CommitmentConfig::finalized()),
        max_supported_transaction_version: Some(0),
    };

    loop{
        let mut retries = 0;
        let latest_slot = rpc_client.get_slot_with_commitment(CommitmentConfig::finalized()); //get latest finalized slot
        match latest_slot{
            Ok(slot) => {
                println!("\nLatest Slot: {}", slot);
                
                let latest_block = rpc_client.get_block_with_config(slot, config); //get block at latest slot
                loop{
                    match latest_block{
                        Ok(encoded_confirmed_block) =>{ //parse block info
                            println!("Block at slot {} found and information retrieved. Parsing...",slot);
                            parse_block(encoded_confirmed_block);
                            break;
                        }
                        Err(ref err) =>{ //will retry block retrieval until timeout
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

        //check iteration and wait
        iter += 1;
        if iter >= MAX_ITER {break;}
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

#[tokio::main]
async fn main() {
    listen_to_slots().await;
}
