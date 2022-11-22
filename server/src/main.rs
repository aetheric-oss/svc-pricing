//! Pricing server.
//!
//! This is the entry point for the pricing server that computes the
//! cost of a flight trip.
mod engine;

use crate::engine::get_pricing;
use env_logger::Builder;
use log::{debug, info, LevelFilter};
use pricing_grpc::pricing_server::{Pricing, PricingServer};
use tonic::transport::Server;
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
        info!("Received request: {:?}", request);
        let query = request.into_inner();
        debug!("Getting pricing for query: {:?}", query);
        let price = get_pricing(query);
        debug!("Pricing computed: {}", price);
        let response = pricing_grpc::PricingResponse { price };
        debug!("Returning response: {:?}", response);
        info!("Successfully computed pricing; returning response");
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
    Builder::new().filter_level(LevelFilter::Info).init();

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
    info!("Starting gRPC server at: {}", full_grpc_addr);
    Server::builder()
        .add_service(health_service)
        .add_service(PricingServer::new(pricing))
        .serve(full_grpc_addr)
        .await?;
    info!("gRPC server running at: {}", full_grpc_addr);

    Ok(())
}
