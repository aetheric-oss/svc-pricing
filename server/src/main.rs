//! Main function starting the server and initializing dependencies.

use lib_common::logger::load_logger_config_from_file;
use log::info;
use svc_pricing::*;

#[tokio::main]
#[cfg(not(tarpaulin_include))]
// no_coverage: (R5) Can not be tested in unit test, should be part of integration tests.
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Will use default config settings if no environment vars are found.
    let config = Config::try_from_env()
        .map_err(|e| format!("Failed to load configuration from environment: {}", e))?;

    // Try to load log configuration from the provided log file.
    // Will default to stdout debug logging if the file can not be loaded.
    load_logger_config_from_file(config.log_config.as_str())
        .await
        .or_else(|e| Ok::<(), String>(log::error!("(main) {}", e)))?;

    info!("(main) Server startup.");

    // Spawn the GRPC server for this service
    tokio::spawn(grpc::server::grpc_server(config, None)).await?;

    info!("(main) server shutdown.");

    // Make sure all log message are written/ displayed before shutdown
    log::logger().flush();

    Ok(())
}
