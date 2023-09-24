//! gRPC server implementation

/// module generated from proto/svc-template-rust-grpc.proto
mod grpc_server {
    #![allow(unused_qualifications, missing_docs)]
    tonic::include_proto!("grpc");
}
pub use grpc_server::rpc_service_server::{RpcService, RpcServiceServer};
pub use grpc_server::{
    pricing_request::ServiceType, PricingRequest, PricingRequests, PricingResponse, ReadyRequest,
    ReadyResponse,
};

use crate::shutdown_signal;
use crate::Config;

use std::fmt::Debug;
use std::net::SocketAddr;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

/// pricing engine functions
use crate::engine::*;

/// struct to implement the gRPC server functions
#[derive(Debug, Default, Copy, Clone)]
pub struct ServerImpl {}

#[cfg(not(feature = "stub_server"))]
#[tonic::async_trait]
impl RpcService for ServerImpl {
    /// Get pricing for a given query.
    async fn get_pricing(
        &self,
        request: Request<PricingRequests>,
    ) -> Result<Response<PricingResponse>, Status> {
        grpc_info!("(get_pricing) pricing server.");
        grpc_debug!("(get_pricing) request: {:?}", request);
        let query = request.into_inner();
        let prices = get_pricing(query);
        match prices {
            Ok(prices) => {
                let response = PricingResponse { prices };
                Ok(tonic::Response::new(response))
            }
            Err(e) => {
                let error = tonic::Status::internal(format!("Error getting pricing: {}", e));
                Err(error)
            }
        }
    }

    /// Returns ready:true when service is available
    async fn is_ready(
        &self,
        request: Request<ReadyRequest>,
    ) -> Result<Response<ReadyResponse>, Status> {
        grpc_info!("(is_ready) pricing server.");
        grpc_debug!("(is_ready) request: {:?}", request);
        let response = ReadyResponse { ready: true };
        Ok(Response::new(response))
    }
}

/// Starts the grpc servers for this microservice using the provided configuration
///
/// # Example:
/// ```
/// use svc_pricing::grpc::server::grpc_server;
/// use svc_pricing::Config;
/// async fn example() -> Result<(), tokio::task::JoinError> {
///     let config = Config::default();
///     tokio::spawn(grpc_server(config, None)).await
/// }
/// ```
#[cfg(not(tarpaulin_include))]
pub async fn grpc_server(config: Config, shutdown_rx: Option<tokio::sync::oneshot::Receiver<()>>) {
    grpc_debug!("(grpc_server) entry.");

    // Grpc Server
    let grpc_port = config.docker_port_grpc;
    let full_grpc_addr: SocketAddr = match format!("[::]:{}", grpc_port).parse() {
        Ok(addr) => addr,
        Err(e) => {
            grpc_error!("(grpc_server) Failed to parse gRPC address: {}", e);
            return;
        }
    };

    let imp = ServerImpl::default();
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<RpcServiceServer<ServerImpl>>()
        .await;

    //start server
    grpc_info!(
        "(grpc_server) Starting gRPC services on: {}.",
        full_grpc_addr
    );
    match Server::builder()
        .add_service(health_service)
        .add_service(RpcServiceServer::new(imp))
        .serve_with_shutdown(full_grpc_addr, shutdown_signal("grpc", shutdown_rx))
        .await
    {
        Ok(_) => grpc_info!("(grpc_server) gRPC server running at: {}.", full_grpc_addr),
        Err(e) => {
            grpc_error!("(grpc_server) Could not start gRPC server: {}", e);
        }
    };
}

#[cfg(feature = "stub_server")]
#[tonic::async_trait]
impl RpcService for ServerImpl {
    /// Get pricing for a given query.
    async fn get_pricing(
        &self,
        request: Request<PricingRequests>,
    ) -> Result<Response<PricingResponse>, Status> {
        grpc_info!("(get_pricing MOCK) pricing server.");
        grpc_debug!("(get_pricing MOCK) request: {:?}", request);
        let query = request.into_inner();
        let prices = get_pricing(query);
        match prices {
            Ok(prices) => {
                let response = PricingResponse { prices };
                Ok(Response::new(response))
            }
            Err(e) => {
                let error = Status::internal(format!("Error getting pricing: {}", e));
                Err(error)
            }
        }
    }

    async fn is_ready(
        &self,
        request: Request<ReadyRequest>,
    ) -> Result<Response<ReadyResponse>, Status> {
        grpc_warn!("(is_ready MOCK) pricing server.");
        grpc_debug!("(is_ready MOCK) request: {:?}", request);
        let response = ReadyResponse { ready: true };
        Ok(Response::new(response))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_grpc_server_is_ready() {
        let imp = ServerImpl::default();
        let result = imp.is_ready(Request::new(ReadyRequest {})).await;
        assert!(result.is_ok());
        let result: ReadyResponse = result.unwrap().into_inner();
        assert_eq!(result.ready, true);
    }
}
