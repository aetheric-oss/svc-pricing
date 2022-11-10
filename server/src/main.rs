//! Pricing server.
//!
//! This is the entry point for the pricing server that computes the
//! cost of a flight trip.
mod engine;

use tonic::transport::Server;

use crate::engine::get_pricing;
use pricing_grpc::pricing_server::{Pricing, PricingServer};
/// Pricing Client Library: Client Functions, Structs
pub mod pricing_grpc {
    #![allow(unused_qualifications)]
    include!("grpc.rs");
}

/// Struct that implements the Pricing trait.
///
/// This is the main struct that implements the gRPC service.
#[derive(Default, Debug, Clone, Copy)]
pub struct ArrowPricing {}

// implementing the service trait for our struct
#[tonic::async_trait]
impl Pricing for ArrowPricing {
    /// Get pricing for a given query.
    /// # Arguments
    /// * `request` - the query object needed to produce
    ///   pricing.
    /// # Returns
    /// * `PricingResponse` - containing the cost of the flight trip in
    ///   dollars.
    async fn get_pricing(
        &self,
        request: tonic::Request<pricing_grpc::PricingRequest>,
    ) -> Result<tonic::Response<pricing_grpc::PricingResponse>, tonic::Status> {
        let query = request.into_inner();
        let price = get_pricing(query);
        let response = pricing_grpc::PricingResponse { price };
        Ok(tonic::Response::new(response))
    }

    /// Return true if this server is ready to serve others.
    ///
    /// # Arguments
    /// * `_request` - the query object with no arguments
    /// # Returns
    /// * `ReadyResponse` - Returns true
    async fn is_ready(
        &self,
        _request: tonic::Request<pricing_grpc::ReadyRequest>,
    ) -> Result<tonic::Response<pricing_grpc::ReadyResponse>, tonic::Status> {
        let response = pricing_grpc::ReadyResponse { ready: true };
        Ok(tonic::Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // GRPC Server
    let grpc_port = std::env::var("DOCKER_PORT_GRPC")
        .unwrap_or_else(|_| "50051".to_string())
        .parse::<u16>()
        .unwrap_or(50051);

    let full_grpc_addr = format!("[::]:{}", grpc_port).parse()?;

    // creating a service
    let pricing = ArrowPricing::default();

    // creating the health check service
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<PricingServer<ArrowPricing>>()
        .await;

    //start server
    println!("Starting gRPC server at: {}", full_grpc_addr);
    Server::builder()
        .add_service(health_service)
        .add_service(PricingServer::new(pricing))
        .serve(full_grpc_addr)
        .await?;
    println!("gRPC server running at: {}", full_grpc_addr);

    Ok(())
}
