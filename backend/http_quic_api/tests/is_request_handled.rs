use anyhow::{Context, Result};
use quinn::{Endpoint, crypto::rustls::QuicClientConfig};
use rustls::RootCertStore;
use rustls_pemfile::certs;
use std::{fs::File, io::BufReader, net::ToSocketAddrs, sync::Arc, usize};

#[tokio::test]
async fn test_quic_server_response() -> Result<()> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");
    let server_addr = "127.0.0.1:4433";

    let mut roots = RootCertStore::empty();

    let mut cert_reader = BufReader::new(File::open("certs/local.key")?);
    let cert_chain = certs(&mut cert_reader).collect::<Result<Vec<_>, _>>()?;
    for cert in cert_chain {
        roots.add(cert)?;
    }

    let mut client_crypto = rustls::ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth();

    client_crypto.alpn_protocols.push(b"h3".to_vec());

    let client_config =
        quinn::ClientConfig::new(Arc::new(QuicClientConfig::try_from(client_crypto)?));

    let mut endpoint = Endpoint::client("[::]:0".parse()?)?;
    endpoint.set_default_client_config(client_config);

    let addr = server_addr
        .to_socket_addrs()?
        .next()
        .context("could not resolve server address")?;

    let connection = endpoint
        .connect(addr, "localhost")?
        .await
        .context("failed to connect to server")?;

    let (mut send, mut recv) = connection.open_bi().await?;
    send.write_all(b"ping").await?;
    let _ = send.finish();

    recv.read_to_end(usize::MAX).await?;

    Ok(())
}
