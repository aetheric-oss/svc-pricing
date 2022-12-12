# `svc-pricing`- Software Design Document (SDD)

<center>

<img src="https://github.com/Arrow-air/tf-github/raw/main/src/templates/doc-banner-services.png" style="height:250px" />

</center>

### Metadata

| Item | Description |
| --- | --- |
| Maintainer(s) | [Services Team](https://github.com/orgs/Arrow-air/teams/services) |
| Primary Contact |[GoodluckH](https://github.com/GoodluckH)|

## Overview

Attribute | Description
--- | ---
Status | :yellow_circle: Development

This document details the software implementation of `svc-pricing` (the "pricing module").

The pricing module is responsible for computing the cost for given flight trips. 

*Note: This module is intended to be used by other Arrow micro-services via gRPC.*

*This document is under development as Arrow operates on a pre-revenue and pre-commercial stage. Pricing logics may evolve as per business needs, which may result in architectural/implementation changes to the pricing module.*

## Related Documents

Document | Description
--- | ---
[High-Level Concept of Operations (CONOPS)](https://github.com/Arrow-air/se-services/blob/develop/docs/conops.md) | Overview of Arrow microservices.
[High-Level Interface Control Document (ICD)](https://github.com/Arrow-air/se-services/blob/develop/docs/icd.md) | Interfaces and frameworks common to all Arrow microservices.
[Pricing Model](https://docs.google.com/spreadsheets/d/1mjPtaIn3E5m7r4nyKt_sJKG9BSFm2ty7Gzo7OqERxwo) | Unit economics and pricing mechanism of flights. The core logic of `svc-pricing` is largely derived from the pricing model.
[Uber Elevate White Paper](https://evtol.news/__media/PDFs/UberElevateWhitePaperOct2016.pdf) | Uber's research on UAM operations. Certain economic assumptions are referenced by Arrow's pricing model.
[Concept of Operations - `svc-pricing`](./conops.md) | Concept of Operations for `svc-pricing`.
[Interface Control Document - `svc-pricing`](./icd.md) | Interface Control Document for `svc-pricing`.

## Location

This module is to be deployed and consumed in a server-side environment.

## Module Attributes

Attribute | Applies | Explanation
--- | --- | ---
Safety Critical | No | Pricing is business critical but has no direct impact to the operational safety. 
Realtime | No | Currently, the price of a trip is computed and supplied to customers as an upfront quote. The price is intended to be final and contractually effective upon customers' booking confirmation.<br /><br /> The price computed by `svc-pricing` will become a persistent state associated with a particular flight.<br /><br /> Dynamic pricing can be in realtime, but it will be the responsibility of  other modules.|


## Logic 

### Initialization

This module does not require user-side initialization.

The `main` function in [`/server/src/main.rs`](../server/src/main.rs) will simply spin up the server at the provided port.

### Environment Variables
The only environment variables are the port numbers used to spin up the server.

For the pricing server, `DOCKER_PORT_GRPC` is the port number where the server lives. If not provided, `50051` will be used as a fallback port.

For the client, `HOST_PORT_GRPC` is needed to connect to the pricing server. This env var should be the server's port. If not provided, `50051` will be used as a fallback port. In most cases, one may assume `HOST_PORT_GRPC` to have the same value as `DOCKER_PORT_GRPC`.

### Control Loop

Does not apply. 

This module is stateless and is expected to be called on-demand as a function by other modules through gRPC.

### Cleanup

Does not apply.

## Interface Handlers

See [the ICD](./icd.md) for this microservice.


## Tests


### Unit Tests
