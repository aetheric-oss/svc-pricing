//! Pricing server.
//!
//! This is the entry point for the pricing server that computes the
//! cost of a flight trip.
mod engine;

use tonic::transport::Server;

use crate::engine::get_pricing;
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
impl pricing_grpc::pricing_server::Pricing for ArrowPricing {
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

    // TODO Implement IsReady
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // defining address for our service
    let addr = "[::1]:50051".parse().unwrap();
    // creating a service
    let pricing = ArrowPricing::default();
    println!("Server listening on {}", addr);
    // adding our service to our server.
    Server::builder()
        .add_service(pricing_grpc::pricing_server::PricingServer::new(pricing))
        .serve(addr)
        .await?;
    Ok(())
}
