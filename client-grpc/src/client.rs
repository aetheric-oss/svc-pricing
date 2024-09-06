//! Client Library: Client Functions, Structs, Traits
#![allow(unused_qualifications)]
include!("grpc.rs");

use super::*;

#[cfg(not(feature = "stub_client"))]
use lib_common::grpc::ClientConnect;
use lib_common::grpc::{Client, GrpcClient};
use rpc_service_client::RpcServiceClient;
/// GrpcClient implementation of the RpcServiceClient
pub type PricingClient = GrpcClient<RpcServiceClient<Channel>>;

cfg_if::cfg_if! {
    if #[cfg(feature = "stub_backends")] {
        use svc_pricing::grpc::server::{RpcServiceServer, ServerImpl};
        lib_common::grpc_mock_client!(RpcServiceClient, RpcServiceServer, ServerImpl);
        super::log_macros!("grpc", "app::client::mock::pricing");
    } else {
        lib_common::grpc_client!(RpcServiceClient);
        super::log_macros!("grpc", "app::client::pricing");
    }
}

#[cfg(not(feature = "stub_client"))]
#[async_trait]
impl crate::service::Client<RpcServiceClient<Channel>> for PricingClient {
    type ReadyRequest = ReadyRequest;
    type ReadyResponse = ReadyResponse;
    type PricingRequests = PricingRequests;
    type PricingResponse = PricingResponse;

    async fn is_ready(
        &self,
        request: Self::ReadyRequest,
    ) -> Result<tonic::Response<Self::ReadyResponse>, tonic::Status> {
        grpc_info!("{} client.", self.get_name());
        grpc_debug!("request: {:?}", request);
        self.get_client().await?.is_ready(request).await
    }

    async fn get_pricing(
        &self,
        request: Self::PricingRequests,
    ) -> Result<tonic::Response<Self::PricingResponse>, tonic::Status> {
        grpc_info!("{} client.", self.get_name());
        grpc_debug!("request: {:?}", request);
        self.get_client().await?.get_pricing(request).await
    }
}

#[cfg(feature = "stub_client")]
#[async_trait]
impl crate::service::Client<RpcServiceClient<Channel>> for PricingClient {
    type ReadyRequest = ReadyRequest;
    type ReadyResponse = ReadyResponse;
    type PricingRequests = PricingRequests;
    type PricingResponse = PricingResponse;

    async fn is_ready(
        &self,
        request: Self::ReadyRequest,
    ) -> Result<tonic::Response<Self::ReadyResponse>, tonic::Status> {
        grpc_warn!("(MOCK) {} client.", self.get_name());
        grpc_debug!("(MOCK) request: {:?}", request);
        Ok(tonic::Response::new(ReadyResponse { ready: true }))
    }

    async fn get_pricing(
        &self,
        request: Self::PricingRequests,
    ) -> Result<tonic::Response<Self::PricingResponse>, tonic::Status> {
        grpc_info!("(MOCK) {} client.", self.get_name());
        grpc_debug!("(MOCK) request: {:?}", request);
        let mut prices = vec![];
        for _ in request.requests {
            prices.push(1.20)
        }
        Ok(tonic::Response::new(PricingResponse { prices }))
    }
}

#[cfg(test)]
mod tests {
    use crate::service::Client as ServiceClient;

    use super::*;

    #[tokio::test]
    #[cfg(not(feature = "stub_client"))]
    async fn test_client_connect() {
        let name = "pricing";
        let (server_host, server_port) =
            lib_common::grpc::get_endpoint_from_env("GRPC_HOST", "GRPC_PORT");

        let client: PricingClient = GrpcClient::new_client(&server_host, server_port, name);
        assert_eq!(client.get_name(), name);

        let connection = client.get_client().await;
        println!("{:?}", connection);
        assert!(connection.is_ok());
    }

    #[tokio::test]
    async fn test_client_is_ready_request() {
        let name = "pricing";
        let (server_host, server_port) =
            lib_common::grpc::get_endpoint_from_env("GRPC_HOST", "GRPC_PORT");

        let client: PricingClient = GrpcClient::new_client(&server_host, server_port, name);
        assert_eq!(client.get_name(), name);

        let result = client.is_ready(ReadyRequest {}).await;
        println!("{:?}", result);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().into_inner().ready, true);
    }

    #[tokio::test]
    async fn test_client_get_pricing_request() {
        let name = "pricing";
        let (server_host, server_port) =
            lib_common::grpc::get_endpoint_from_env("GRPC_HOST", "GRPC_PORT");

        let client: PricingClient = GrpcClient::new_client(&server_host, server_port, name);
        assert_eq!(client.get_name(), name);

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
        let result = client
            .get_pricing(PricingRequests {
                requests: vec![query1, query2, query3],
            })
            .await;
        println!("{:?}", result);
        assert!(result.is_ok());
        let result = result.unwrap().into_inner();
        assert_eq!(result.prices.len(), 3);
    }
}
