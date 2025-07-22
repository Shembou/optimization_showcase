pub mod api;
pub mod config;
pub mod logging;
pub mod utils;

use crate::{config::server_config::configure_server, logging::logging::init_tracing};
#[tokio::main]
async fn main() {
    let _log_guard = init_tracing("logs", "app").expect("Failure during logger initialization");
    configure_server().await
}
