

use std::env;
use dotenv::dotenv;
use std::convert::TryFrom;
use std::string::String;

use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};
use ethers::{prelude::*};

mod body;
use crate::body::body_format;

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

                    let transaction_str = body_format::format_transaction_list(transaction);

                    body_format::template_block_body(block_number, block_hash, block_gas_used, timestamp, transaction_str)
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

#[get("/block/{number}")]
async fn block_data(path: web::Path<(u32,)>) -> HttpResponse {
    // let url = env::var("URL")?;
    let block_number = path.into_inner().0;

    if block_number == 0 {
        return HttpResponse::Ok().body(format!("error"));
    }

    return HttpResponse::Ok().body(format!("block"));
}

// #[get("/erc20")]
// async fn erc20() -> impl Responder {}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(block_data)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
