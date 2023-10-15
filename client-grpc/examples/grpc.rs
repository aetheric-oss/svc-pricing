//! gRPC client implementation.
//!
//! This is an example of a gRPC client that connects to Arrow's pricing
//! server.
//! gRPC client implementation

use lib_common::grpc::get_endpoint_from_env;
use svc_pricing_client_grpc::prelude::*;

/// Example svc-pricing-client-grpc
///
/// Assuming the server is running on localhost:50051, this method calls
/// `client.get_pricing` and should receive a valid response from the
/// server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_GRPC");
    let client = PricingClient::new_client(&host, port, "pricing");
    println!("Client created.");
    println!(
        "NOTE: Ensure the server is running on {} or this example will fail.",
        client.get_address()
    );

    let response = client.is_ready(pricing::ReadyRequest {}).await?;

    println!("RESPONSE={:?}", response.into_inner());

    let query1 = pricing::PricingRequest {
        service_type: 0,
        weight_kg: 100.0,
        distance_km: 100.0,
    };
    let query2 = pricing::PricingRequest {
        service_type: 0,
        weight_kg: 100.0,
        distance_km: 100.0,
    };
    let query3 = pricing::PricingRequest {
        service_type: 0,
        weight_kg: 100.0,
        distance_km: 100.0,
    };
    let pricing_requests = vec![query1, query2, query3];
    let query = pricing::PricingRequests {
        requests: pricing_requests,
    };

    let response = client.get_pricing(query).await?;

    println!("(main) RESPONSE={:?}", response.into_inner());

    Ok(())
}
