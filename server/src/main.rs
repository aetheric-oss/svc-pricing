//! Main function starting the server and initializing dependencies.

use log::info;
use svc_pricing::*;

/// Main entry point: starts gRPC Server on specified address and port
#[tokio::main]
#[cfg(not(tarpaulin_include))]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("(svc-pricing) server startup.");

    // Will use default config settings if no environment vars are found.
    let config = Config::try_from_env().unwrap_or_default();

    println!("{:?}", config);
    // Start Logger
    let log_cfg: &str = config.log_config.as_str();
    if let Err(e) = log4rs::init_file(log_cfg, Default::default()) {
        panic!(
            "(logger) could not parse log config {} found in config {:?}: {}.",
            log_cfg, config, e
        );
    }

    tokio::spawn(grpc::server::grpc_server(config, None)).await?;

    info!("(svc-pricing) server shutdown.");
    Ok(())
}
