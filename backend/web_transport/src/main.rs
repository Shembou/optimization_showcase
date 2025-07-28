use rcgen::{CertifiedKey, generate_simple_self_signed};
use wtransport::tls::{Certificate, CertificateChain, PrivateKey};
use wtransport::{Endpoint, Identity, ServerConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subject_alt_names = vec![
        "web_transport.localhost".to_string(),
        "localhost".to_string(),
    ];
    let CertifiedKey { cert, signing_key } =
        generate_simple_self_signed(subject_alt_names).unwrap();

    let private_key = PrivateKey::from_der_pkcs8(signing_key.serialize_der());
    let certificate = Certificate::from_der(cert.der().to_vec())?;

    let cert_chain = CertificateChain::new(vec![certificate]);

    let identity = Identity::new(cert_chain, private_key);

    let config = ServerConfig::builder()
        .with_bind_default(4433)
        .with_identity(identity)
        .build();

    let server = Endpoint::server(config)?;

    println!("Server listening on https://localhost:4433");

    loop {
        let incoming_session = server.accept().await;
        let session_request = incoming_session.await?;
        println!("New session: authority={:?}", session_request.authority());
        let connection = session_request.accept().await?;
        println!("Connection established with id {}", connection.stable_id());
    }
}
