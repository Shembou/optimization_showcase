pub mod store;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_file = "./proto/store.proto";
    tonic_build::configure()
        .build_server(false)
        .out_dir("./src")
        .compile_protos(&[proto_file], &["proto"])?;

    Ok(())
}
