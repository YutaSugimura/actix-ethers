use std::env;
use dotenv::dotenv;
use std::convert::TryFrom;
use std::string::String;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use ethers::prelude::*;

fn format_transaction(transactions: Vec<H256>) -> String {
    let mut tx_str: String = String::from("<div><ul>");

    for tx in transactions {
        let str = format!("{:02X}", tx);
        let url = format!("'https://etherscan.io/tx/0x{}'", &str);
        tx_str = tx_str.to_string() + "<li><a href=" + &*url + ">" + "0x" + &*str + "</a></li>";
    }

    tx_str = tx_str + "</ul></div>";
    tx_str
}

fn format_html(block_number: U64, block_hash: String, block_gas_used: U256, timestamp: U256, transaction_str: String) -> String {
    format!("\
        <html lang='en' />\
            <head>\
                <meta charset='UTF-8'>\
            </head>\
            <body>\
                <div>\
                    <h1>Ethereum latest block data</h1>\
                    <h2>block number: {}</h2>\
                    <div>\
                        <p>block hash: {}</p>\
                        <p>gas used: {}</p>\
                        <p>timestamp: {}</p>\
                    </div>\

                    <div>\
                        <h3>transactions</h3>
                        {}\
                    </div>\
                </div>\
            </body>\
        </html>
    ", block_number, block_hash, block_gas_used, timestamp, transaction_str)
}

#[tokio::main]
async fn latest_block(url: String) -> String {
    let provider = Provider::<Http>::try_from(url);

    match provider {
        Err(_) => format!("error"),
        Ok(provider) => {
            let block_number = match provider.get_block_number().await {
                Err(_) => U64::from(0),
                Ok(num) => num,
            };

            if block_number == U64::from(0) {
                format!("error");
            }

            match provider.get_block(block_number).await {
                Err(_) => format!("errpr"),
                Ok(block_result) => {
                    let block_hash = match &block_result {
                        Some(data) => {
                            match data.hash {
                                Some(hash) => format!("0x{:02X}", hash),
                                None => String::from("Error"),
                            }
                        },
                        None => String::from("unknown"),
                    };

                    let block_gas_used = match &block_result {
                        Some(data) => data.gas_limit,
                        None => U256::from(0),
                    };

                    let timestamp = match &block_result {
                        Some(data) => data.timestamp,
                        None => U256::from(0),
                    };

                    let transaction = match block_result {
                        Some(data) => data.transactions,
                        None => Vec::new(),
                    };

                    let transaction_str = format_transaction(transaction);

                    format_html(block_number, block_hash, block_gas_used, timestamp, transaction_str)
                },
            }
        }
    }
}

#[get("/")]
async fn index() -> impl Responder {
    match env:: var("URL") {
        Err(_e) => HttpResponse::Ok().body("Something went wrong"),
        Ok(url) => {
            let block_number = latest_block(url);
            HttpResponse::Ok().body(block_number)
        },
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(index)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
