//! Client Library: Client Functions, Structs, Traits
/// gRPC object traits to provide wrappers for grpc functions
#[tonic::async_trait]
pub trait Client<T>
where
    Self: Sized + lib_common::grpc::Client<T> + lib_common::grpc::ClientConnect<T>,
    T: Send + Clone,
{
    /// The type expected for ReadyRequest structs.
    type ReadyRequest;
    /// The type expected for ReadyResponse structs.
    type ReadyResponse;
    /// The type expected for PricingRequests structs.
    type PricingRequests;
    /// The type expected for PricingResponse structs.
    type PricingResponse;

    /// Returns a [`tonic::Response`] containing a [`ReadyResponse`](Self::ReadyResponse)
    /// Takes an [`ReadyRequest`](Self::ReadyRequest).
    ///
    /// # Errors
    ///
    /// Returns [`tonic::Status`] with [`Code::Unknown`](tonic::Code::Unknown) if
    /// the server is not ready.
    ///
    /// # Examples
    /// ```
    /// use lib_common::grpc::get_endpoint_from_env;
    /// use svc_pricing_client_grpc::prelude::*;
    ///
    /// async fn example () -> Result<(), Box<dyn std::error::Error>> {
    ///     let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_GRPC");
    ///     let connection = PricingClient::new_client(&host, port, "pricing");
    ///     let response = connection.is_ready(pricing::ReadyRequest {}).await?;
    ///     println!("RESPONSE={:?}", response.into_inner());
    ///     Ok(())
    /// }
    /// ```
    async fn is_ready(
        &self,
        request: Self::ReadyRequest,
    ) -> Result<tonic::Response<Self::ReadyResponse>, tonic::Status>;

    /// Returns a [`tonic::Response`] containing a [`PricingResponse`](Self::PricingResponse)
    /// Takes an [`PricingRequests`](Self::PricingRequests).
    ///
    /// # Errors
    ///
    /// Returns [`tonic::Status`] with [`Code::Internal`](tonic::Code::Internal) if
    /// no pricing could be determined.
    ///
    /// # Examples
    /// ```
    /// use lib_common::grpc::get_endpoint_from_env;
    /// use svc_pricing_client_grpc::prelude::*;
    ///
    /// async fn example () -> Result<(), Box<dyn std::error::Error>> {
    ///     let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_GRPC");
    ///     let connection = PricingClient::new_client(&host, port, "pricing");
    ///     let query = pricing::PricingRequests {
    ///         requests: vec![pricing::PricingRequest {
    ///             service_type: 0,
    ///             weight_kg: 100.0,
    ///             distance_km: 100.0,
    ///         }]
    ///     };
    ///     let response = connection.get_pricing(query).await?;
    ///     println!("RESPONSE={:?}", response.into_inner());
    ///     Ok(())
    /// }
    /// ```
    async fn get_pricing(
        &self,
        request: Self::PricingRequests,
    ) -> Result<tonic::Response<Self::PricingResponse>, tonic::Status>;
}
