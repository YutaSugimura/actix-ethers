

pub mod body_format {
    use std::string::String;
    use ethers::types::{U64, U256, H256};

    pub fn format_transaction_list(transactions: Vec<H256>) -> String {
        let mut tx_str: String = String::from("<div><ul>");

        for tx in transactions {
            let str = format!("{:02X}", tx);
            let url = format!("'https://etherscan.io/tx/0x{}'", &str);
            tx_str = tx_str.to_string() + "<li><a href=" + &*url + ">" + "0x" + &*str + "</a></li>";
        }

        tx_str = tx_str + "</ul></div>";
        tx_str
    }

    pub fn template_block_body(block_number: U64, block_hash: String, block_gas_used: U256, timestamp: U256, transaction_str: String) -> String {
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
}