use std::net::SocketAddr;

use axum::Router;
use axum::routing::get;

use crate::api::home::hello;
use crate::utils::certs::get_certs_config;

pub async fn configure_server() {
    let rustls_config = get_certs_config().await;
    let app = Router::<()>::new().route("/", get(hello));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let mut server = axum_server::bind_rustls(addr, rustls_config);
    server.http_builder().http2().enable_connect_protocol();
    server.serve(app.into_make_service()).await.unwrap();
}
