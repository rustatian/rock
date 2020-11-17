#![warn(rust_2018_idioms)]

use actix_web::client::Client;
use core::profile::buffer::{Buffer, Decoder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut client = Client::default();

    // Create request builder and send request
    let response = client.get("http://www.rust-lang.org")
        .header("User-Agent", "actix-web/3.0")
        .send()     // <- Send request
        .await;     // <- Wait for response

    println!("Response: {:?}", response);

    let mut v = vec![1, 2];
    let b = Buffer::decode(&mut v)?;
    Ok(())

    // b.
}
