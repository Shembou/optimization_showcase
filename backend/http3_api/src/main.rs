use std::fmt::Error;

const key_path: &str = "../certs/localhost/key.pem";
const cert_path: &str = "../certs/localhost/cert.pem";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cert = Some(cert_path);
}