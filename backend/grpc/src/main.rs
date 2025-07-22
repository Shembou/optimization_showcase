use crate::{greeter::greeter_server::GreeterServer, server::MyGreeter};
use std::net::SocketAddr;
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;

pub mod greeter;
pub mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3002));
    let greeter = GreeterServer::new(MyGreeter::default());

    const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("greeter_descriptor");

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build_v1()
        .unwrap();

    Server::builder()
        .accept_http1(true)
        .layer(GrpcWebLayer::new())
        .add_service(greeter)
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}
