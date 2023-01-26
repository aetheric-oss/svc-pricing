//! gRPC client implementation.
//!
//! This is an example of a gRPC client that connects to Arrow's pricing
//! server.

use svc_pricing_client::pricing_grpc::{
    pricing_client::PricingClient, PricingRequest, PricingRequests, ReadyRequest,
};

/// Example svc-pricing-client.
///
/// Assuming the server is running on localhost:50051, this method calls
/// `client.get_pricing` and should receive a valid response from the
/// server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let grpc_port = std::env::var("HOST_PORT_GRPC").unwrap_or_else(|_| "50051".to_string());
    let mut client = PricingClient::connect(format!("http://[::1]:{grpc_port}")).await?;

    let request = tonic::Request::new(ReadyRequest {});
    let response = client.is_ready(request).await;

    let mut ok = true;
    if response.is_err() {
        ok = false;
        println!("IsReady: FAIL");
    } else {
        println!("IsReady: PASS");
    }

    if ok {
        println!("\u{1F9c1} All endpoints responded!");
    } else {
        eprintln!("\u{2620} Errors");
    }

    let query1 = PricingRequest {
        service_type: 0,
        weight_kg: 100.0,
        distance_km: 100.0,
    };
    let query2 = PricingRequest {
        service_type: 0,
        weight_kg: 100.0,
        distance_km: 100.0,
    };
    let query3 = PricingRequest {
        service_type: 0,
        weight_kg: 100.0,
        distance_km: 100.0,
    };
    let pricing_requests = vec![query1, query2, query3];
    let query = PricingRequests {
        requests: pricing_requests,
    };

    let response = client.get_pricing(query).await?;

    println!("RESPONSE={:?}", response.into_inner());

    Ok(())
}
