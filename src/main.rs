use tokio::io;
use tokio::net::{TcpListener, TcpStream};

use futures::future::try_join;
use futures::FutureExt;
use std::error::Error;
use tokio::fs::File;
use tokio::prelude::*;

#[tokio::main]
async fn main() {
    let proxies = get_proxies().await;

    let listen_addr = std::env::var("LISTEN_ADDR").unwrap_or_else(|_| "127.0.0.1:8100".to_string());

    println!("Listening on: {}", listen_addr);

    let mut listener = TcpListener::bind(listen_addr)
        .await
        .expect("Unable to listen socket");

    let mut i = 0;
    while let Ok((inbound, _)) = listener.accept().await {
        let server_addr = proxies[i].clone();
        i = (i + 1) % (proxies.len() - 1);

        let transfer = transfer(inbound, server_addr).map(|r| {
            if let Err(e) = r {
                println!("Failed to transfer; error={}", e);
            }
        });

        tokio::spawn(transfer);
    }
}

async fn transfer(mut inbound: TcpStream, proxy_addr: String) -> Result<(), Box<dyn Error>> {
    let mut outbound = TcpStream::connect(proxy_addr).await?;

    let (mut ri, mut wi) = inbound.split();
    let (mut ro, mut wo) = outbound.split();

    let client_to_server = io::copy(&mut ri, &mut wo);
    let server_to_client = io::copy(&mut ro, &mut wi);

    try_join(client_to_server, server_to_client).await?;

    Ok(())
}

async fn get_proxies() -> Vec<String> {
    let proxies_path = std::env::var("PROXY_PATH")
        .expect("Please specify absolute path in PROXY_PATH environment variable");

    let mut proxies_file_handle = File::open(proxies_path)
        .await
        .expect("Unable to open proxy file");

    let mut proxies_file_content = vec![];

    proxies_file_handle
        .read_to_end(&mut proxies_file_content)
        .await
        .expect("Unable to read proxy file");

    let proxies_ =
        String::from_utf8(proxies_file_content).expect("Cant convert prixes content to string");

    let mut proxies = vec![];

    proxies_
        .split("\n")
        .for_each(|proxy| proxies.push(String::from(proxy)));

    proxies
}
