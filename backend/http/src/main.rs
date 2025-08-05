use crate::{
    config::{api::configure_rest_api, grpc::configure_gprc_server},
    tracing::init_tracing,
};

pub mod api;
pub mod config;
pub mod graphql;
pub mod proto;
pub mod tracing;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _log_guard = init_tracing("logs", "app").expect("Failure during logger initialization");
    let rest_api_handle = tokio::spawn(configure_rest_api());
    let grpc_handle = tokio::spawn(configure_gprc_server());
    let (rest_result, grpc_result) = tokio::join!(rest_api_handle, grpc_handle);
    rest_result??;
    grpc_result??;
    Ok(())
}