use std::env;

#[cfg(test)]

#[test]
fn is_certs_directory_correct() {
    let current_working_directory = env::current_dir().expect("Failed to get current working directory");
    let certs_path = current_working_directory.join("../certs/localhost/");
    let key_path = certs_path.join("key.pem");
    let cert_path = certs_path.join("cert.pem");
    assert!(
        key_path.exists(),
        "key.pem not found at {}",
        key_path.display()
    );
    assert!(
        cert_path.exists(),
        "certs.pem not found at {}",
        certs_path.display()
    );
}
