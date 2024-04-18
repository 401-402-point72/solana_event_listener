// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
#![allow(clippy::result_large_err)]
// #![allow(deprecated)] 
// #![allow(unused_imports)]
// #![allow(unused_variables)]

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::{config::Region, Client};
// use aws_sdk_s3::{Client};
use serde_json::Value;
use std::env;

// use aws_types::sdk_config::SdkConfig as AwsSdkConfig;
use aws_config::from_env;

pub async fn init_connection() -> (String, Client) {
    // Pull in environment variables
    dotenv::dotenv().ok();

    let bucket_name = &env::var("BUCKET_NAME").unwrap();
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");

    println!("Bucket Name: {}", bucket_name);
    println!("Region: {}", region_provider.region().await.unwrap());

    // region_provider somehow gets borrowed so gotta print first ... weird rust bs
        // aws_config is having some issues(probably bc of the patched crate?)
    let config= aws_config::from_env().region(region_provider).load().await;
    // let config: aws_types::sdk_config::SdkConfig = aws_config::from_env().region(region_provider).load().await;
    // dbg!(&config);
    let client = Client::new(&config);


    (bucket_name.to_string(), client)
}

// Upload a file to a bucket.
// #[tokio::main]
pub async fn upload_object(client: &Client, bucket: &str, block: &Value, slot: &u64) -> () {
    // println!("{:#}", block);

    let key = slot.to_string();
    // Convert json object to rust native byte stream and then aws byte stream
    let rust_bytestream = serde_json::to_vec(&block).unwrap();
    let aws_bytestream = ByteStream::from(rust_bytestream);

    // Grab block number as indexable key
    // let key = match block["block_height"].as_str() {
    //     Some(value) => value,
    //     None => {
    //         println!("Block number not found or is not a string");
    //         return;
    //     }
    // };

    // Store object in bucket ... YAY!
    let _response = client
        .put_object()
        .bucket(bucket)
        .key(&key)
        .body(aws_bytestream)
        .send()
        .await;

    println!("Object uploaded to S3 with key: {}", key);
    ()
}
