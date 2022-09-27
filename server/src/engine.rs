//! Implementation of pricing logics based on the ride type.

use crate::grpc::PricingRequest;

/// Get pricing for a given query
///
/// # Arguments
/// * `request` - PricingRequest
pub fn get_pricing(query: PricingRequest) -> f32 {
    match query.service_type {
        0 => get_cargo_pricing(query),
        1 => get_rideshare_pricing(query),
        2 => get_charter_pricing(query),
        _ => 0.0,
    }
}

// ------------------------------------------------------------------
// Cargo pricing assumptions
// Expect these constants to be pulled from svc-storage in the future
// https://docs.google.com/spreadsheets/d/1mjPtaIn3E5m7r4nyKt_sJKG9BSFm2ty7Gzo7OqERxwo
// ------------------------------------------------------------------

/// Take off and landing cost in dollars.
const CARGO_TOL_COST: f32 = 2.8;
/// Cruise speed in kilometers per hour.
const CARGO_CRUISE_SPEED: f32 = 240.0;
/// Electricity (kw) needed to power every hour of cruise flight.
const CARGO_CRUISE_POWER_CONSUMPTION: f32 = 71.0;
/// Electricity cost per kilowatt hour in dollars.
const CARGO_ELECTRICITY_COST: f32 = 0.3335;
/// Depreciation rate of the aircraft in dollars per hour.
const CARGO_DEPRECIATION_RATE: f32 = 10.5;
/// Repair and maintenance cost in dollars per hour.
const CARGO_REPAIR_AND_MAINTENANCE_RATE: f32 = 0.3 * CARGO_DEPRECIATION_RATE;

// ------------------------------------------------------------------
// private functions
// ------------------------------------------------------------------

/// Pricing for cargo.
///
/// Pricing is based on distance for now. The unit economics are modeled
/// after [Project
/// Apollo](https://docs.google.com/spreadsheets/d/1mjPtaIn3E5m7r4nyKt_sJKG9BSFm2ty7Gzo7OqERxwo).
///
/// # Arguments
/// * `query` - A [`PricingRequest`] struct that contains information
///   needed to compute the pricing.
///
/// # Returns
/// * `f32` - The cost of the flight trip in dollars.
fn get_cargo_pricing(query: PricingRequest) -> f32 {
    let distance = query.distance;
    let trip_duration = distance / CARGO_CRUISE_SPEED;
    let trip_cruise_cost = trip_duration * CARGO_ELECTRICITY_COST * CARGO_CRUISE_POWER_CONSUMPTION;
    let depreciation_cost = trip_duration * CARGO_DEPRECIATION_RATE;
    let repair_and_maintenance_cost = trip_duration * CARGO_REPAIR_AND_MAINTENANCE_RATE;
    CARGO_TOL_COST + trip_cruise_cost + depreciation_cost + repair_and_maintenance_cost
}

/// TODO: Pricing for rideshare.
fn get_rideshare_pricing(query: PricingRequest) -> f32 {
    //TODO
    get_cargo_pricing(query)
}

/// TODO: Pricing for charter.
fn get_charter_pricing(query: PricingRequest) -> f32 {
    //TODO
    get_cargo_pricing(query)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cargo_pricing() {
        let query = PricingRequest {
            service_type: 0,
            distance: 160.934,
        };
        let price = get_cargo_pricing(query);
        assert_eq!((price * 10.0).round() / 10.0, 27.8);
    }
}
