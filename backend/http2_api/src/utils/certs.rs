use std::{env, path::PathBuf};

use axum_server::tls_rustls::RustlsConfig;
pub async fn get_certs_config() -> RustlsConfig {
    let cwd = env::current_dir().expect("Failed to get current working directory");
    let certs_path = cwd.join("./certs/localhost/");
    let key_path = certs_path.join("key.pem");
    let cert_path = certs_path.join("cert.pem");
    return RustlsConfig::from_pem_file(PathBuf::from(cert_path), PathBuf::from(key_path))
        .await
        .unwrap();
}
