#![warn(rust_2018_idioms)]

use actix_web::client::Client;

#[actix_web::main]
async fn main() {
    let mut client = Client::default();

    // Create request builder and send request
    let response = client.get("http://www.rust-lang.org")
        .header("User-Agent", "actix-web/3.0")
        .send()     // <- Send request
        .await;     // <- Wait for response

    println!("Response: {:?}", response);
}
