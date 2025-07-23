use crate::{greeter::greeter_server::GreeterServer, server::MyGreeter};
use std::{env, net::SocketAddr};
use tonic::transport::{Identity, Server, ServerTlsConfig};
use tonic_web::GrpcWebLayer;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

pub mod greeter;
pub mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");
    let port = env::var("PORT").unwrap_or_else(|e| {
        info!(
            "PORT environment variable not set. Defaulting port to 3000. Err: {}",
            e
        );
        "3002".to_string()
    });
    let addr = SocketAddr::from((
        [0, 0, 0, 0],
        port.parse()
            .expect("Failed during parsing string to number"),
    ));
    let greeter = GreeterServer::new(MyGreeter::default());

    const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("greeter_descriptor");

    let cert = include_str!("../certs/local.crt");
    let key = include_str!("../certs/local.key");

    let identity = Identity::from_pem(cert, key);

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build_v1()
        .unwrap();

    let tls_config = ServerTlsConfig::new().identity(identity);

    Server::builder()
        .tls_config(tls_config)?
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .accept_http1(true)
        .layer(GrpcWebLayer::new())
        .add_service(greeter)
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}
