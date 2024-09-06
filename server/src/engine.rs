//! Implementation of pricing logics based on the service type.

use log::debug;
use snafu::prelude::Snafu;

use crate::grpc::server::{PricingRequest, PricingRequests, ServiceType};

/// Errors that can occur when getting pricing.
#[derive(Snafu, Copy, Clone, Debug, PartialEq)]
pub enum PricingError {
    /// Pricing requests contain multiple service types.
    #[snafu(display("All pricing requests must have the same service type"))]
    MultipleServiceTypes,

    /// Request contains negative weight.
    #[snafu(display("Request contains negative weight"))]
    NegativeWeight,

    /// Request contains negative distance.
    #[snafu(display("Request contains negative distance"))]
    NegativeDistance,

    /// No pricing requests were provided.
    #[snafu(display("No pricing requests were provided"))]
    NoRequests,

    /// Unknown service type error.
    #[snafu(display("Unknown service type; cannot parse service type"))]
    UnknownServiceType,
}

/// Get pricing given a [`PricingRequests`], which contains an array of
/// [`PricingRequest`]s.
///
/// The array of [`PricingRequest`]s is used to allow pricing for
/// multiple legs of a trip to be calculated.
///
/// # Notes
/// The `service_type` field of all [`PricingRequest`]s in the array
/// must be the same. It means the pricing service currently does not
/// allow for a trip that involves different types of services.
///
/// This may change in the future as per business requirements.
///
/// # Returns
/// If no errors occur, returns a vector of prices in dollars,
/// corresponding to the order of the input requests.
pub fn get_pricing(query: PricingRequests) -> Result<Vec<f32>, PricingError> {
    let requests = query.requests;
    debug!(
        "(get_pricing) Getting pricing for {:?} requests.",
        requests.len()
    );
    check_pricing_requests(&requests)?;
    let mut prices = Vec::new();
    match ServiceType::try_from(requests[0].service_type) {
        Ok(ServiceType::Cargo) => {
            debug!("(get_pricing) Cargo pricing.");
            for request in requests {
                prices.push(get_cargo_pricing(request));
            }
            Ok(prices)
        }
        Ok(ServiceType::Rideshare) => {
            // debug!("Rideshare pricing");
            // for request in requests {
            //     prices.push(get_rideshare_pricing(request));
            // }
            // Ok(prices)
            debug!("(get_pricing) Rideshare pricing not supported.");
            Err(PricingError::UnknownServiceType)
        }
        Ok(ServiceType::Charter) => {
            // debug!("Charter pricing");
            // for request in requests {
            //     prices.push(get_charter_pricing(request));
            // }
            // Ok(prices)
            debug!("(get_pricing) Charter pricing not supported.");
            Err(PricingError::UnknownServiceType)
        }
        Err(e) => {
            debug!("(get_pricing) Error parsing service type: {}", e);
            Err(PricingError::UnknownServiceType)
        }
    }
}

/// Check that all pricing requests have the same service type.
fn check_pricing_requests(requests: &[PricingRequest]) -> Result<(), PricingError> {
    debug!("(check_pricing_requests) entry.");
    if requests.is_empty() {
        return Err(PricingError::NoRequests);
    }
    let service_type = requests[0].service_type;
    for request in requests {
        if request.service_type != service_type {
            return Err(PricingError::MultipleServiceTypes);
        } else if request.weight_kg < 0.0 {
            return Err(PricingError::NegativeWeight);
        } else if request.distance_km < 0.0 {
            return Err(PricingError::NegativeDistance);
        }
    }
    Ok(())
}

// ------------------------------------------------------------------
// Cargo pricing assumptions
// Expect these constants to be pulled from svc-storage in the future
// ------------------------------------------------------------------

/// Take off and landing cost in dollars.
const CARGO_TOL_COST_USD: f32 = 2.8;
/// Cruise speed in kilometers per hour.
const CARGO_CRUISE_SPEED_KM_PER_HR: f32 = 240.0;
/// Electricity (kw) needed to power every hour of cruise flight.
const CARGO_CRUISE_POWER_CONSUMPTION_KW: f32 = 71.0;
/// Electricity cost per kilowatt hour in dollars.
const CARGO_ELECTRICITY_COST_USD_PER_KWH: f32 = 0.3335;
/// Depreciation rate of the aircraft in dollars per hour.
const CARGO_DEPRECIATION_RATE_USD_PER_HR: f32 = 10.5;
/// Repair and maintenance cost in dollars per hour.
const CARGO_REPAIR_AND_MAINTENANCE_RATE_USD_PER_HR: f32 = 0.3 * CARGO_DEPRECIATION_RATE_USD_PER_HR;

// ------------------------------------------------------------------
// private functions
// ------------------------------------------------------------------

/// Pricing for cargo.
fn get_cargo_pricing(query: PricingRequest) -> f32 {
    debug!(
        "(get_cargo_pricing) Getting cargo pricing for query: {:?}",
        query
    );
    let distance = query.distance_km;
    debug!(
        "(get_cargo_pricing) Cargo take off and landing cost: {}",
        CARGO_TOL_COST_USD
    );
    debug!("(get_cargo_pricing) Distance: {}", distance);
    let trip_duration = distance / CARGO_CRUISE_SPEED_KM_PER_HR;
    debug!("(get_cargo_pricing) Trip duration: {}", trip_duration);
    let trip_cruise_cost =
        trip_duration * CARGO_ELECTRICITY_COST_USD_PER_KWH * CARGO_CRUISE_POWER_CONSUMPTION_KW;
    debug!("(get_cargo_pricing) Trip cruise cost: {}", trip_cruise_cost);
    let depreciation_cost = trip_duration * CARGO_DEPRECIATION_RATE_USD_PER_HR;
    debug!(
        "(get_cargo_pricing) Depreciation cost: {}",
        depreciation_cost
    );
    let repair_and_maintenance_cost = trip_duration * CARGO_REPAIR_AND_MAINTENANCE_RATE_USD_PER_HR;
    debug!(
        "(get_cargo_pricing) Repair and maintenance cost: {}",
        repair_and_maintenance_cost
    );
    let total_cost =
        CARGO_TOL_COST_USD + trip_cruise_cost + depreciation_cost + repair_and_maintenance_cost;
    debug!("(get_cargo_pricing) Total cost: {}", total_cost);
    total_cost
}

// /// TODO(R5) Pricing for rideshare.
// fn get_rideshare_pricing(query: PricingRequest) -> f32 {
//     0.5 * get_cargo_pricing(query)
// }

// /// TODO(R5) Pricing for charter.
// fn get_charter_pricing(query: PricingRequest) -> f32 {
//     2.0 * get_cargo_pricing(query)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cargo_pricing() {
        let query = PricingRequests {
            requests: vec![PricingRequest {
                service_type: ServiceType::Cargo as i32,
                weight_kg: 100.0,
                distance_km: 100.0,
            }],
        };
        let prices = get_pricing(query).unwrap();
        assert_eq!(prices[0], 18.353542);
    }

    #[test]
    fn test_get_rideshare_pricing() {
        let query = PricingRequests {
            requests: vec![PricingRequest {
                service_type: ServiceType::Rideshare as i32,
                weight_kg: 100.0,
                distance_km: 100.0,
            }],
        };
        let error = get_pricing(query).unwrap_err();
        assert_eq!(error, PricingError::UnknownServiceType);
    }

    #[test]
    fn test_get_charter_pricing() {
        let query = PricingRequests {
            requests: vec![PricingRequest {
                service_type: ServiceType::Charter as i32,
                weight_kg: 100.0,
                distance_km: 100.0,
            }],
        };
        let error = get_pricing(query).unwrap_err();
        assert_eq!(error, PricingError::UnknownServiceType);
    }

    #[test]
    fn test_multiple_leg_pricing() {
        let query1 = PricingRequest {
            service_type: ServiceType::Cargo as i32,
            weight_kg: 100.0,
            distance_km: 100.0,
        };
        let query2 = PricingRequest {
            service_type: ServiceType::Cargo as i32,
            weight_kg: 100.0,
            distance_km: 50.0,
        };
        let query3 = PricingRequest {
            service_type: ServiceType::Cargo as i32,
            weight_kg: 100.0,
            distance_km: 10.0,
        };
        let pricing_requests = vec![query1, query2, query3];
        let query = PricingRequests {
            requests: pricing_requests,
        };
        let prices = get_pricing(query).unwrap();
        let total = prices.iter().fold(0.0, |acc, x| acc + x);

        let price_leg_1 = get_cargo_pricing(query1);
        let price_leg_2 = get_cargo_pricing(query2);
        let price_leg_3 = get_cargo_pricing(query3);

        assert_eq!(prices.len(), 3);
        assert_eq!(prices[0], price_leg_1);
        assert_eq!(prices[1], price_leg_2);
        assert_eq!(prices[2], price_leg_3);
        assert_eq!(total, price_leg_1 + price_leg_2 + price_leg_3);
    }

    #[test]
    fn test_invalid_multiple_service_type() {
        let query1 = PricingRequest {
            service_type: ServiceType::Cargo as i32,
            weight_kg: 100.0,
            distance_km: 100.0,
        };
        let query2 = PricingRequest {
            service_type: ServiceType::Rideshare as i32,
            weight_kg: 100.0,
            distance_km: 100.0,
        };
        let pricing_requests = vec![query1, query2];
        let query = PricingRequests {
            requests: pricing_requests,
        };
        let error = get_pricing(query).unwrap_err();
        assert_eq!(error, PricingError::MultipleServiceTypes);
    }

    #[test]
    fn test_invalid_service_type() {
        let query = PricingRequest {
            service_type: 3,
            weight_kg: 100.0,
            distance_km: 100.0,
        };
        let prices = get_pricing(PricingRequests {
            requests: vec![query],
        });
        assert_eq!(prices, Err(PricingError::UnknownServiceType));
    }

    #[test]
    fn test_invalid_weight() {
        let query = PricingRequest {
            service_type: ServiceType::Cargo as i32,
            weight_kg: -1.0,
            distance_km: 100.0,
        };
        let prices = get_pricing(PricingRequests {
            requests: vec![query],
        });
        assert_eq!(prices, Err(PricingError::NegativeWeight));
    }

    #[test]
    fn test_invalid_distance() {
        let query = PricingRequest {
            service_type: ServiceType::Cargo as i32,
            weight_kg: 100.0,
            distance_km: -1.0,
        };
        let prices = get_pricing(PricingRequests {
            requests: vec![query],
        });
        assert_eq!(prices, Err(PricingError::NegativeDistance));
    }

    #[test]
    fn test_pricing_request_empty_requests() {
        let empty_query = PricingRequests { requests: vec![] };
        let prices = get_pricing(empty_query);
        assert_eq!(prices, Err(PricingError::NoRequests));
    }
}
