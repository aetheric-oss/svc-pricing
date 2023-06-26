![Arrow Banner](https://github.com/Arrow-air/tf-github/raw/main/src/templates/doc-banner-services.png)

# Software Design Document (SDD) - `svc-pricing` 

## :telescope: Overview

This document details the software implementation of `svc-pricing` (the "pricing module").

The pricing module is responsible for computing the cost for given flight trips. 

*Note: This module is intended to be used by other Arrow micro-services via gRPC.*

*This document is under development as Arrow operates on a pre-revenue and pre-commercial stage. Pricing logics may evolve as per business needs, which may result in architectural/implementation changes to the pricing module.*

### Metadata

| Attribute     | Description                                                       |
| ------------- |-------------------------------------------------------------------|
| Maintainer(s) | [Services Team](https://github.com/orgs/Arrow-air/teams/services) |
| Stuckee       | [@GoodluckH](https://github.com/GoodluckH)                        |
| Status        | Development                                                       |

## :books: Related Documents

Document | Description
--- | ---
--- | ---
[High-Level Concept of Operations (CONOPS)](https://github.com/Arrow-air/se-services/blob/develop/docs/conops.md) | Overview of Arrow microservices.
[High-Level Interface Control Document (ICD)](https://github.com/Arrow-air/se-services/blob/develop/docs/icd.md) | Interfaces and frameworks common to all Arrow microservices.
[Requirements - `svc-pricing`](https://nocodb.arrowair.com/dashboard/#/nc/view/045288a8-3875-4429-bdaa-9f578275adef) | Requirements and user stories for this microservice.
[Concept of Operations - `svc-pricing`](./conops.md) | Defines the motivation and duties of this microservice.
[Interface Control Document (ICD) - `svc-pricing`](./icd.md) | Defines the inputs and outputs of this microservice.
[Pricing Model](https://docs.google.com/spreadsheets/d/1mjPtaIn3E5m7r4nyKt_sJKG9BSFm2ty7Gzo7OqERxwo) | Unit economics and pricing mechanism of flights. The core logic of `svc-pricing` is largely derived from the pricing model.
[Uber Elevate White Paper](https://evtol.news/__media/PDFs/UberElevateWhitePaperOct2016.pdf) | Uber's research on UAM operations. Certain economic assumptions are referenced by Arrow's pricing model.

## :dna: Module Attributes

Attribute | Applies | Explanation
--- | --- | ---
Safety Critical | No | Pricing is business critical but has no direct impact to the operational safety. 
Realtime | No | Currently, the price of a trip is computed and supplied to customers as an upfront quote. The price is intended to be final and contractually effective upon customers' booking confirmation.<br /><br /> The price computed by `svc-pricing` will become a persistent state associated with a particular flight.<br /><br /> Dynamic pricing can be in realtime, but it will be the responsibility of  other modules.|

## :gear: Logic

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

## :speech_balloon: gRPC Handlers

See [the ICD](./icd.md) for this microservice.
