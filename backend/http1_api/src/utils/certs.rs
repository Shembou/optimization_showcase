use std::{env, fs::File, io::BufReader};

pub struct Certification {
    pub key: BufReader<File>,
    pub cert: BufReader<File>,
}

pub fn get_certs() -> Certification {
    let cwd = env::current_dir().expect("Failed to get current working directory");
    let certs_path = cwd.join("../certs/localhost/");
    let key_path = certs_path.join("key.pem");
    let cert_path = certs_path.join("cert.pem");
    let certs = Certification {
        key: BufReader::new(File::open(key_path).unwrap()),
        cert: BufReader::new(File::open(cert_path).unwrap()),
    };
    return certs;
}
