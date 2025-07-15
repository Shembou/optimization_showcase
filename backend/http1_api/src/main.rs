pub mod api;
pub mod config;
pub mod utils;

use crate::config::server_config::configure_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    configure_server().await
}
