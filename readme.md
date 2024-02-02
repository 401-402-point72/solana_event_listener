# Solana Event Listener Overview
This repo contains our solana event listener, which listens on and stores information from recent slots/blocks on the solana blockchain. Written in Rust.
# How it works - src/main.rs
- Our event listener gets the most recently finalized slot on the solana blockchain. It takes this slot number and attempts to retrieve the block present in that slot. If a block is unable to be retrieved, it will retry until a time-out. 
- Upon successful retrieval, the block information and it's transactions will be parsed and stored in a data structure as well as printed to the console.
- The information stored in the data structure, once gathered, will be sent to an AWS database for logging.
# Libraries Used
- solana-sdk: used to connect to solana rpc endpoint
- solana-client: used for various solana methods, namely CommitmentConfig, which lets us specify a commitment level when trying to acquire the most recent slot.
- tokio: used for asynchronous operations and timing/delays. 
- chrono: used for date and time formatting
## TODO
- investigate and fix RPC response error -32015: Transaction version (0) is not supported by the requesting client. Please try the request again with the following configuration parameter: "maxSupportedTransactionVersion": 0
- parse fields in retrieved blocks for desired information
- Create data structure to hold the information from each slot/block to easily store in database
- store in database in addition to printing to console