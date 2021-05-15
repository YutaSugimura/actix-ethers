use std::env;
use dotenv::dotenv;
use std::convert::TryFrom;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use ethers::prelude::*;

fn format_html(str: String) -> String {
    format!("<div><h1>block explorer</h1><h2>block number: {}</h2></div>", str)
}

#[tokio::main]
async fn latest_block_number(url: String) -> String {
    let provider = Provider::<Http>::try_from(url);

    match provider {
        Err(_) => format!("error"),
        Ok(provider) => {
            let block_number = provider.get_block_number().await;

            match block_number {
                Err(_) => format!("error"),
                Ok(block_number) => {
                    format_html(format!("{}", block_number))
                }
            }
        }
    }
}

#[get("/")]
async fn index() -> impl Responder {
    match env:: var("URL") {
        Err(_e) => HttpResponse::Ok().body("Something went wrong"),
        Ok(url) => {
            let block_number = latest_block_number(url);
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
