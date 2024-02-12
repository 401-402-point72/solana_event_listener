# Solana Event Listener Overview
This repo contains our solana event listener, which listens on and stores information from recent slots/blocks on the solana blockchain. Written in Rust.
# How it works - src/main.rs
- Our event listener gets the most recently finalized slot on the solana blockchain. It takes this slot number and attempts to retrieve the block present in that slot. If a block is unable to be retrieved, it will retry until a time-out. 
- Upon successful retrieval, the block information and it's transactions will be parsed and stored in an AWS database as well as printed to the console.
# Libraries Used
- solana-sdk: used to connect to solana rpc endpoint
- solana-client: used for various solana methods, namely CommitmentConfig, which lets us specify a commitment level when trying to acquire the most recent slot.
- solana-transaction-status: used for EncodedConfirmedBlock struct
- tokio: used for asynchronous operations and timing/delays. 
- chrono: used for date and time formatting
## IMPORTANT TO NOTE
- DEVNET HAS BEEN DOWN SINCE FEB 9th SO OUR EVENT LISTENER ONLY RETRIEVES THE MOST RECENT BLOCK REPEATEDLY. WILL REMOVE NOTICE ONCE DEVNET IS BACK UP
## TODO
- investigate and fix RPC response error -32015: Transaction version (0) is not supported by the requesting client. Please try the request again with the following configuration parameter: "maxSupportedTransactionVersion": 0
- in regards to above: use get_block_with_config and set the config's max supported transaction version to be 0.
see https://docs.rs/solana-client/latest/solana_client/rpc_client/struct.RpcClient.html#method.get_block_with_config
- ^^^THIS IS DONE, JUST NEED TO TEST ONCE DEVNET IS BACK ONLINE
- parse fields in retrieved blocks for desired information
- store in database in addition to printing to console