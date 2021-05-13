use ethers::prelude::*;
use std::time::Duration;
use std::convert::TryFrom;

use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let url = env::var("URL").expect("URL is not found");

    let provider = Provider::<Http>::try_from(url)?;
    let result = provider.get_block_number().await;

    match result {
        Ok(block_number) => {
            let result_block = provider.get_block(block_number).await?;

            match result_block {
                Some(block_detail) => {
                    println!("block_hash={:?}", &block_detail.hash);
                    println!("block_gas_used={}", &block_detail.gas_used);
                    println!("timestamp={}", &block_detail.timestamp);
                    println!("tx start");
                    for tx in &block_detail.transactions {
                        println!("{}", tx);
                    }
                    println!("tx end");
                },
                None => println!("None"),
            }
        },
        Err(e) => {}
    }


    // let url_ws = env::var("URL").expect("URL is not found");
    // let ws = Ws::connect(url_ws).await?;
    // let provider = Provider::new(ws).interval(Duration::from_millis(2000));
    // let mut stream = provider.watch_blocks().await?.stream();
    // while let Some(block_hash) = stream.next().await {
    //     let mut block_number = provider.get_block_number().await?.to_string();
    //     dbg!(block_number, block_hash);
    // }

    Ok(())
}