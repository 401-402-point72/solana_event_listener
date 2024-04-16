# Solana Event Listener Overview
This repo contains our solana event listener, which listens on and stores information from recent slots/blocks on the solana blockchain. Written in Rust.
# How it works - src/main.rs
- Our event listener gets the most recently finalized slot on the solana blockchain. It takes this slot number and attempts to retrieve the block present in that slot. If a block is unable to be retrieved, it will retry until a time-out. 
- Upon successful retrieval the block data, it's transaction data, and it's reward data will be parsed into json objects and compiled into a json file that contains all the information in the block. This will be used to store the data in our AWS database.
# AWS S3
To access the S3 bucket for storage You must create a .env file in the root of this project with the bucket name specified, in this case: rustbucketsolana
```
BUCKET_NAME=rustbucketsolana
```
You must also have an .aws folder with a "credentials" file in your home directory with the aws access and secret key to verify credentials
```
[Admin2]
aws_access_key_id = "access key"
aws_secret_access_key = "secret key"
[default]
aws_access_key_id = "access key"
aws_secret_access_key = "secret key"
```
# Libraries Used - See cargo.toml for dependencies list and version numbers
- solana-sdk: 
    - used to connect to solana rpc endpoint. https://docs.rs/solana-sdk/latest/solana_sdk/index.html
- solana-client: 
    - used for various solana methods, namely CommitmentConfig, which lets us specify a commitment level when trying to acquire the most recent slot. https://docs.rs/solana-client/latest/solana_client/index.html
- solana-transaction-status: 
    - used for EncodedConfirmedBlock/UiConfirmedBlock struct. https://docs.rs/solana-transaction-status/latest/solana_transaction_status/index.html
- tokio: 
    - used for asynchronous operations and timing/delays. https://docs.rs/tokio/latest/tokio/ 
- chrono: 
    - used for date and time formatting. https://docs.rs/chrono/latest/chrono/
- serde:
    - used for json functionality. https://docs.rs/serde/latest/serde/
## DEVNET STATUS
- https://explorer.solana.com/?cluster=devnet
## MAINNET STATUS
- https://explorer.solana.com/?cluster=mainnet-beta 
