fn main() {
    cc::Build::new()
        .cpp(true)
        .files([
            "cpp/stdsoap2.cpp",
            "cpp/soapClient.cpp",
            "cpp/soapC.cpp",
            "cpp/externs/calculator.cpp",
        ])
        .define("SOAP_CLIENT_LIB", None)
        .include("cpp")
        .compile("libsoapclient.a");
}
