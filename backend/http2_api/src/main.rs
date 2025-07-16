pub mod api;
pub mod config;
pub mod utils;

use crate::config::server_config::configure_server;
#[tokio::main]
async fn main() {
    configure_server().await
}
