use std::{fs::File, io::BufReader, net::SocketAddr, sync::Arc};

use anyhow::{Context, Result};
use quinn::crypto::rustls::QuicServerConfig;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls_pemfile::{certs, pkcs8_private_keys};
use tracing::{error, info, info_span, Instrument as _};


struct Configuration {
    cert_path: &'static str,
    key_path: &'static str
}

impl Configuration {
    fn prepare_certificates(&self) -> Result<(Vec<CertificateDer<'static>>, PrivateKeyDer<'static>)> {
        // Read and parse certificate chain
        let mut cert_reader = BufReader::new(File::open(self.cert_path)?);
        let cert_chain = certs(&mut cert_reader).collect::<Result<Vec<_>, _>>()?;

        // Read and parse private key (PKCS#8)
        let mut key_reader = BufReader::new(File::open(self.key_path)?);
        let mut keys = pkcs8_private_keys(&mut key_reader);
        let key = match keys.next() {
            Some(result) => result?,
            None => anyhow::bail!("No private key found"),
        };
        let key = PrivateKeyDer::Pkcs8(key);

        Ok((cert_chain, key))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    rustls::crypto::ring::default_provider()
    .install_default()
    .expect("Failed to install rustls crypto provider");
    tracing_subscriber::fmt::init();

    const ALPN_QUIC_HTTP: &[&[u8]] = &[b"h3", b"hq-29"];

    let cert_path = "certs/local.crt";
    let key_path = "certs/local.key";
    let config = Configuration {
        cert_path,
        key_path
    };

    let (cert, key) = config.prepare_certificates()?;
    let addr = SocketAddr::from(([0,0,0,0], 4433));

    let mut server_crypto = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert,key)?;
    server_crypto.alpn_protocols = ALPN_QUIC_HTTP.iter().map(|&x| x.into()).collect();

    let mut server_config = quinn::ServerConfig::with_crypto(Arc::new(QuicServerConfig::try_from(server_crypto)?));

    let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
    transport_config.max_concurrent_uni_streams(0_u8.into());
    
    let endpoint = quinn::Endpoint::server(server_config, addr)?;
    info!("listening on {}", endpoint.local_addr()?);

    while let Some(conn) = endpoint.accept().await {
        info!("connection incoming");
        let fut = handle_connection(conn);
        tokio::spawn(async move {
            if let Err(e) = fut.await {
                error!("connection failed: {reason}", reason = e.to_string())
            }
        });
    }

    Ok(())
}

async fn handle_connection(conn: quinn::Incoming) -> Result<()> {
    let connection = conn.await?;
    let span = info_span!(
        "connection",
        remote = %connection.remote_address(),
        protocol = %connection
            .handshake_data()
            .unwrap()
            .downcast::<quinn::crypto::rustls::HandshakeData>().unwrap()
            .protocol
            .map_or_else(|| "<none>".into(), |x| String::from_utf8_lossy(&x).into_owned())
    );
    async {
        info!("established");

        // Each stream initiated by the client constitutes a new request.
        loop {
            let stream = connection.accept_bi().await;
            let stream = match stream {
                Err(quinn::ConnectionError::ApplicationClosed { .. }) => {
                    info!("connection closed");
                    return Ok(());
                }
                Err(e) => {
                    return Err(e.into());
                }
                Ok(s) => s,
            };
            let fut = handle_request(stream);
            tokio::spawn(
                async move {
                    if let Err(e) = fut.await {
                        error!("failed: {reason}", reason = e.to_string());
                    }
                }
                .instrument(info_span!("request")),
            );
        }
    }
    .instrument(span)
    .await
}


async fn handle_request(
    (mut send, _recv): (quinn::SendStream, quinn::RecvStream),
) -> Result<()> {
    send.write_all(b"Hello World")
        .await
        .context("failed to send response")?;
    // Gracefully terminate the stream
    send.finish().context("failed to finish stream")?;
    info!("complete");
    Ok(())
}