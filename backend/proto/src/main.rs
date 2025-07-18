use std::net::SocketAddr;

use tonic::transport::Server;

use crate::{server::StoreInventory, store::inventory_server::InventoryServer};

pub mod server;
pub mod store;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3002));
    let inventory = StoreInventory::default();
    let proto_file = "./proto/store.proto";

    tonic_build::configure()
        .build_server(true)
        .out_dir("./src")
        .compile_protos(&[proto_file], &["proto"])?;

    Server::builder()
        .add_service(InventoryServer::new(inventory))
        .serve(addr)
        .await?;
    Ok(())
}
