use anyhow::{Context, Result};
use quinn::{Endpoint, crypto::rustls::QuicClientConfig};
use rustls::RootCertStore;
use rustls_pemfile::certs;
use tokio::time::sleep;
use std::{env, fs::File, io::BufReader, net::SocketAddr, process::{Child, Command}, sync::Arc, time::Duration, usize};

#[tokio::test]
async fn is_establishing_connection() -> Result<()> {
    let mut server_process = spawn_backend_server()?;

    // Wait for server to start up (basic wait, for demo purposes)
    sleep(Duration::from_secs(2)).await;
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");
    let port = env::var("PORT").unwrap_or_else(|e| {
        println!("{e}");
        String::from("4433")
    });
    let addr: SocketAddr = format!("127.0.0.1:{}", port).parse()?;

    let mut roots = RootCertStore::empty();

    let mut cert_reader = BufReader::new(File::open("certs/local.crt")?);
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


    let connection = endpoint
        .connect(addr, "localhost")?
        .await
        .context("failed to connect to server")?;

    let (mut send, mut recv) = connection.open_bi().await?;
    send.write_all(b"ping").await?;
    let _ = send.finish();

    recv.read_to_end(usize::MAX).await?;

    server_process.kill().context("Failed to kill backend process")?;
    Ok(())
}

fn spawn_backend_server() -> Result<Child> {
    let mut cmd = Command::new("cargo");
    cmd.args(&["run", "."]);  // <-- change this to your binary name

    // Optionally pass env vars to the backend
    cmd.env("PORT", "4433");

    // Redirect stdout/stderr if you want
    cmd.stdout(std::process::Stdio::null());
    cmd.stderr(std::process::Stdio::null());

    let child = cmd.spawn().context("Failed to spawn backend process")?;
    Ok(child)
}