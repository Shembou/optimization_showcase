use std::{env, fs, net::SocketAddr};

use tonic::transport::{Identity, Server, ServerTlsConfig};
use tonic_web::GrpcWebLayer;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

use crate::proto::{greeter::greeter_server::GreeterServer, server::MyGreeter};

pub async fn configure_gprc_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    info!("binding socket address");
    let port = env::var("GRPC_PORT").unwrap_or_else(|e| {
        info!(
            "GRPC_PORT environment variable not set. Defaulting port to 3003. Err: {}",
            e
        );
        "3003".to_string()
    });
    let port_number: u16 = port.parse()?;
    let addr = SocketAddr::from(([0, 0, 0, 0], port_number));
    let greeter = GreeterServer::new(MyGreeter::default());

    const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("greeter_descriptor");

    let cert = fs::read("certs/local.crt")?;
    let key = fs::read("certs/local.key")?;

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
