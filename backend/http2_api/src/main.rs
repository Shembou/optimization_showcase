pub mod api;
pub mod config;
pub mod utils;
pub mod logging;

use crate::config::server_config::configure_server;
#[tokio::main]
async fn main() {
    configure_server().await
}
