//! Re-export of used objects

pub use super::client as pricing;
pub use super::service::Client as PricingServiceClient;
pub use pricing::PricingClient;

pub use lib_common::grpc::Client;
