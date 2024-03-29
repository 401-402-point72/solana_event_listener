//uncomment to turn off warnings
#![allow(deprecated)] 
#![allow(unused_imports)]
#![allow(unused_variables)]


use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcBlockConfig;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_transaction_status::{
    UiTransaction, 
    EncodedTransaction,
    EncodedConfirmedBlock, 
    Reward, 
    EncodedTransactionWithStatusMeta, 
    UiConfirmedBlock,
    UiTransactionEncoding,
    TransactionDetails};
use chrono::{LocalResult, TimeZone, Utc, DateTime};
use serde_json::json;

const MAX_ITER : i32 = 10; //number of slots to get and try to retrieve a block from before terminating
const MAX_RETRIES : i32 = 5; //number of retries to retrieve block from the current slot before timing out

fn parse_transactions(transactions: Vec<EncodedTransactionWithStatusMeta>) { //parses block transaction data vector
    println!("Total number of transactions: {}", transactions.len());
    for transaction in transactions{
        //jsonify transaction info
        let transaction_info = json!({
            "transaction": transaction.transaction,
            "meta": transaction.meta,
            "version": transaction.version,
        });

        //print, (store in database later)
        //println!("Parsed Transaction: {}", transaction_info);
    }
}

fn parse_rewards(rewards: Vec<Reward>) { //parses block reward data vector
    println!("Total number of rewards: {}", rewards.len());
    for reward in rewards {
        //jsonify reward info
        let reward_info = json!({
            "pubkey": reward.pubkey,
            "lamports": reward.lamports,
            "post balance": reward.post_balance,
            "reward type": reward.reward_type,
            "commission": reward.commission,
        });

        //print, (store in database later)
        println!("Parsed Reward: {}", reward_info);
    }
}

fn parse_block(encoded_confirmed_block: UiConfirmedBlock){ //parses info in confirmed/finalized block    
    let block_time = encoded_confirmed_block.block_time.unwrap(); // unwrap time into int (assume time exists, may need to change to match to handle errors)
    let block_time_str = DateTime::<Utc>::from_utc(chrono::NaiveDateTime::from_timestamp(block_time as i64, 0), Utc).to_rfc2822(); // convert int time to string time using chrono

    // jsonify block info
    let block_info = json!({
        "blockhash": encoded_confirmed_block.blockhash,
        "prev_blockhash": encoded_confirmed_block.previous_blockhash,
        "parent slot": encoded_confirmed_block.parent_slot,
        "block height": encoded_confirmed_block.block_height,
        "block time(str)": block_time_str,
        "block time(int)": block_time
    });
    //print,(store to database later)
    println!("Parsed block: {}", block_info);

    //check for/get transactions
    if let Some(transactions) = encoded_confirmed_block.transactions{
        parse_transactions(transactions);
    }
    else{
        println!("No transactions in this block.");
    }

    //check for/get rewards
    if let Some(rewards) = encoded_confirmed_block.rewards{
        parse_rewards(rewards);
    }
    else{
        println!("No rewards in this block.");
    }
}

async fn listen_to_slots() {
    let rpc_url = "https://api.mainnet-beta.solana.com".to_string(); // testing mainnet
    let rpc_client = RpcClient::new(rpc_url); //connect to rpc endpoint
    let mut iter = 0;
    let config = RpcBlockConfig { //set up config to retrieve blocks
        encoding: Some(UiTransactionEncoding::Base58),
        transaction_details: Some(TransactionDetails::Full),
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
