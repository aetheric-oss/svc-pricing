//! build script to generate .rs from .proto

///generates .rs files in src directory
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = "../proto";
    let proto_file = &format!("{}/svc-pricing-grpc.proto", proto_dir);

    let server_config = tonic_build::configure()
        .type_attribute("ReadyRequest", "#[derive(Copy, Eq)]")
        .type_attribute("ReadyResponse", "#[derive(Copy, Eq)]")
        .type_attribute("PricingRequest", "#[derive(Copy)]");
    // .type_attribute("PricingResponse", "#[derive(Copy)]");
    let client_config = server_config.clone();

    // Build the Client
    client_config
        .client_mod_attribute("grpc", "#[cfg(not(tarpaulin_include))]")
        .build_server(false)
        .out_dir("../client-grpc/src/")
        .compile(&[proto_file], &[proto_dir])?;

    // Build the Server
    server_config
        .build_client(false)
        .compile(&[proto_file], &[proto_dir])?;

    println!("(build) cargo:rerun-if-changed={}", proto_file);

    Ok(())
}
