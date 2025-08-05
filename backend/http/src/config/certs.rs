use axum_server::tls_rustls::RustlsConfig;
use rustls::{
    ServerConfig,
    pki_types::{CertificateDer, PrivateKeyDer},
};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::{fs::File, io::BufReader, sync::Arc};

pub async fn get_certs_config() -> RustlsConfig {
    let cwd = std::env::current_dir().unwrap();
    let cert_path = cwd.join("certs/local.crt");
    let key_path = cwd.join("certs/local.key");
    let cert_file = &mut BufReader::new(File::open(cert_path).expect("Cannot open cert file"));
    let key_file = &mut BufReader::new(File::open(key_path).expect("Cannot open key file"));

    let cert_chain = certs(cert_file)
        .map(|r| r.map(CertificateDer::from))
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to read certificate");

    let mut keys = pkcs8_private_keys(key_file)
        .map(|r| r.map(PrivateKeyDer::from))
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to read private key");

    let mut config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, keys.remove(0))
        .expect("bad certificate/key");

    config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];

    RustlsConfig::from_config(Arc::new(config))
}
