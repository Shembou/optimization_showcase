use std::{
    error::Error,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use crate::common::make_server_endpoint;
mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let server_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 5001);
    let (endpoint, _server_cert) = make_server_endpoint(server_addr)?;
    println!("[server] listening on {}", server_addr);

    while let Some(incoming_conn) = endpoint.accept().await {
        tokio::spawn(async move {
            let conn = incoming_conn.await.unwrap();
            println!(
                "[server] connection accepted: addr={}",
                conn.remote_address()
            );
        });
    }
    Ok(())
}