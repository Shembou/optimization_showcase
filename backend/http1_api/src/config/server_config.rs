use actix_web::{App, HttpServer};

use crate::{
    api::{common::healthcheck, home::hello},
    utils::certs::get_certs,
};
pub async fn configure_server() -> std::io::Result<()> {
    let mut certs = get_certs();

    let tls_certs = rustls_pemfile::certs(&mut certs.cert)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let tls_key = rustls_pemfile::pkcs8_private_keys(&mut certs.key)
        .next()
        .unwrap()
        .unwrap();

    let tls_config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(tls_certs, rustls::pki_types::PrivateKeyDer::Pkcs8(tls_key))
        .unwrap();

    return HttpServer::new(|| App::new().service(hello).service(healthcheck))
        .bind_rustls_0_23(("127.0.0.1", 8443), tls_config)?
        .run()
        .await;
}
