/// Are you Ready?
///
/// No arguments
#[derive(Copy, Eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadyRequest {
}
/// I'm Ready
#[derive(Copy, Eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadyResponse {
    /// Indicate if the service is ready to accept requests.
    #[prost(bool, tag="1")]
    pub ready: bool,
}
/// Get the price for a type of service.
///
/// Two required fields:
/// - `service_type`: the type of service. 0 = cargo, 1 = rideshare, 2 =
///    charter
/// - `distance`: the distance of the trip in km
#[derive(Copy)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PricingRequest {
    /// service type
    /// 0 = cargo
    /// 1 = rideshare
    /// 2 = charter
    #[prost(enumeration="pricing_request::ServiceType", tag="1")]
    pub service_type: i32,
    /// distance in kilometers
    ///
    /// weight in kg - Not in use for now
    ///
    /// conversations are ongoing to determine how weight
    /// impacts pricing
    ///
    /// required float weight_kg = 3;
    #[prost(float, tag="2")]
    pub distance: f32,
}
/// Nested message and enum types in `PricingRequest`.
pub mod pricing_request {
    /// Service type
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ServiceType {
        /// Cargo service that can transport goods.
        Cargo = 0,
        /// Rideshare service that can transport passengers.
        Rideshare = 1,
        /// Charter service that can be reserved for a specific trip.
        Charter = 2,
    }
    impl ServiceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ServiceType::Cargo => "CARGO",
                ServiceType::Rideshare => "RIDESHARE",
                ServiceType::Charter => "CHARTER",
            }
        }
    }
}
/// Price for a service
#[derive(Copy)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PricingResponse {
    /// price in dollars
    #[prost(float, tag="1")]
    pub price: f32,
}
/// Generated server implementations.
pub mod pricing_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with PricingServer.
    #[async_trait]
    pub trait Pricing: Send + Sync + 'static {
        async fn get_pricing(
            &self,
            request: tonic::Request<super::PricingRequest>,
        ) -> Result<tonic::Response<super::PricingResponse>, tonic::Status>;
        async fn is_ready(
            &self,
            request: tonic::Request<super::ReadyRequest>,
        ) -> Result<tonic::Response<super::ReadyResponse>, tonic::Status>;
    }
    /// Pricing for different services: cargo, rideshare, and charter
    #[derive(Debug)]
    pub struct PricingServer<T: Pricing> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Pricing> PricingServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PricingServer<T>
    where
        T: Pricing,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/grpc.Pricing/GetPricing" => {
                    #[allow(non_camel_case_types)]
                    struct GetPricingSvc<T: Pricing>(pub Arc<T>);
                    impl<T: Pricing> tonic::server::UnaryService<super::PricingRequest>
                    for GetPricingSvc<T> {
                        type Response = super::PricingResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PricingRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_pricing(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPricingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grpc.Pricing/IsReady" => {
                    #[allow(non_camel_case_types)]
                    struct IsReadySvc<T: Pricing>(pub Arc<T>);
                    impl<T: Pricing> tonic::server::UnaryService<super::ReadyRequest>
                    for IsReadySvc<T> {
                        type Response = super::ReadyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).is_ready(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = IsReadySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Pricing> Clone for PricingServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Pricing> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Pricing> tonic::server::NamedService for PricingServer<T> {
        const NAME: &'static str = "grpc.Pricing";
    }
}
