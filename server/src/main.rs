//! Pricing server.
//!
//! This is the entry point for the pricing server that computes the
//! cost of a flight trip.
mod engine;
mod grpc;

use tonic::transport::Server;

use crate::engine::get_pricing;
use crate::grpc::{
    pricing_server::{Pricing, PricingServer},
    PricingResponse, QueryPricing,
};

/// Struct that implements the Pricing trait.
///
/// This is the main struct that implements the gRPC service.
#[derive(Default)]
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
        request: tonic::Request<QueryPricing>,
    ) -> Result<tonic::Response<PricingResponse>, tonic::Status> {
        let query = request.into_inner();
        let price = get_pricing(query);
        let response = PricingResponse { price };
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
        .add_service(PricingServer::new(pricing))
        .serve(addr)
        .await?;
    Ok(())
}
