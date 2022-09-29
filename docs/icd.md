# Interface Control Document (ICD) - `svc-pricing`

<center>

<img src="https://github.com/Arrow-air/tf-github/raw/main/src/templates/doc-banner-services.png" style="height:250px" />

</center>

## Overview

This document details the common software interfaces required by all services in the Arrow ecosystem.

Each service may add additional interfaces.

Attribute | Description
--- | ---
Status | Draft

## Related Documents

Document | Description
--- | ---
:construction: Requirements & User Stories :construction: | Requirements and user stories

## Frameworks

See the Services ICD.

## REST

### Files

TODO

### Authentication

See the Services ICD.

### Endpoints

None. This is an internal service expected to be consumed by other Arrow services.


## gRPC

### Files

These interfaces are defined in a protocol buffer file, `svc-pricing-grpc.proto`.

### Integrated Authentication & Encryption

See Services ICD.

### gRPC Server Methods ("Services")

gRPC server methods are called "services", unfortunately name clashing with the broader concept of web services.

| Service | Description |
| ---- | ---- |
| `IsReady` | Returns a message indicating if this service is ready for requests.<br>Similar to a health check, if a server is not "ready" it could be considered dead by the client making the request.

### gRPC Client Methods ("Requests")

| Request | Description |
| ------    | ------- |
| `FlightQuery` | A message to the svc-scheduler in particular
