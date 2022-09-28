//! gRPC client implementation.
//!
//! This is an example of a gRPC client that connects to Arrow's pricing
//! server.

use cargo_client::pricing_grpc;
use pricing_grpc::pricing_client::PricingClient;
use pricing_grpc::PricingRequest;

/// Example svc-pricing-client.
///
/// Assuming the server is running on localhost:50051, this method calls
/// `client.get_pricing` and should receive a valid response from the
/// server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PricingClient::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(PricingRequest {
        service_type: 0, // 0 for cargo, 1 for rideshare, 2 for charter
        distance: 100.0, // in km
    });

    let response = client.get_pricing(request).await?;

    println!("RESPONSE={:?}", response.into_inner());

    Ok(())
}
