# Concept of Operations - `svc-pricing`

<center>

<img src="https://github.com/Arrow-air/tf-github/raw/main/src/templates/doc-banner-services.png" style="height:250px" />

</center>

Attribute | Description
--- | ---
Maintainer | [@Arrow-air/services](https://github.com/orgs/Arrow-air/teams)
Status | Experiment
  
## Important Notes

Arrow is still at a pre-revenue stage without a definitive business model. This Concept of Operations document assumes Arrow to bear the cost of building and operating aircraft.

However, due to the significant financial and engineering resources required to build out certified, commercial scale vehicles, it is likely that we operate as a marketplace platform like Uber where original equipment manufacturers (OEMs) supply vehicles. Therefore, the pricing mechanism will be dramatically different than what `svc-pricing` implements.

 Some preliminary ideas about this business model will be outlined in this document under [Business Model Discussion](#business-model-discussion).

## Overview

`svc-pricing` provides the ticket price (in USD) for a given trip. The pricing model currently supports three use cases: cargo, rideshare, and charter.

The price supplied and presented to the customers through GUI clients should be the definitive charge to be billed.

## Related Documents

Document | Description
--- | ---
[High-Level Concept of Operations (CONOPS)](https://github.com/Arrow-air/se-services/blob/develop/docs/conops.md) | Overview of Arrow microservices.
[High-Level Interface Control Document (ICD)](https://github.com/Arrow-air/se-services/blob/develop/docs/icd.md) | Interfaces and frameworks common to all Arrow microservices.
[Interface Control Document (ICD) - `svc-pricing`](./icd.md) | Defines the inputs and outputs of this microservice.
[Pricing Model](https://docs.google.com/spreadsheets/d/1mjPtaIn3E5m7r4nyKt_sJKG9BSFm2ty7Gzo7OqERxwo) | Unit economics and pricing mechanism of flights. The core logic of `svc-pricing` is largely derived from the pricing model.
[Uber Elevate White Paper](https://evtol.news/__media/PDFs/UberElevateWhitePaperOct2016.pdf) | Uber's research on UAM operations. Certain economic assumptions are referenced by Arrow's pricing model.

## Motivation

Arrow strives to provide compelling value propositions to customers. Having a predictable and refined pricing model builds trust through the increased transparency.

Under the current business model, we expect Arrow to incur heavy capital expenditures (CapEx) to build out a fleet of aircraft to make our network valuable. The pricing model, at its current stage, attempts to recoup the CapEx investment in building, operating, and maintaining VTOL vehicles. 

Although we are assuming a non-profit business, the pricing of each individual trip will make up a large part of Arrow's revenue. For now, the unit economics should yield a near 0% profit margin, but in the future, there may be a hurdle return on invested capital (ROIC) on a per trip basis. Therefore, a responsive pricing service is vital to the organization's financial success.

## Needs, Goals and Objectives of Envisioned System

The system should accept key inputs like distance, weight, etc. to compute the pricing of a given trip.

The system should accept requests and return responses.

*Note, the following goals have not been achieved by the current implementation of the pricing system.*

The system should pull assumptions like electricity rates from external services in reasonably fast way. 

The system should be agnostic of geography as external services should supply key assumptions pertinent to local currencies.

## External Interfaces
See the [ICD](icd.md) for this microservice.

## Proposed Capabilities
|Scenario| Description|
--- | ---
|Private Charter | Provides pricing for a bespoke flight. This service should be charged at a premium because it interrupts `svc-scheduler`'s optimization for fleet utilization.
Cargo | Provides pricing for transporting eligible payloads. It should factor in dates, distance, and weight as major drivers for pricing.
Rideshare | Provides pricing for passenger carrying flights. It should factor in the real-time supply and demand for particular routes, pilot salaries, seat classes, etc. The pricing should be very similar to airlines.
Promotional | Provides pricing given a promotional campaign. This would usually apply a discount to the quoted price.  

## Modes of Operation
|Mode|Description|
| --- | --- |
Nominal | Supplies pricing to clients as normal.
Unavailable | The service is down.


## Operational Scenarios, Use Cases and/or Design Reference Missions

See the [High-Level CONOPS](https://github.com/Arrow-air/se-services/blob/develop/docs/conops.md).

## Support Environment

See the [High-Level CONOPS](https://github.com/Arrow-air/se-services/blob/develop/docs/conops.md).

## Business Model Discussion

### Vertical Integration
The current assumed business model is  bearing the cost of building and operating aircraft.

Under this business model, the motivation of pricing is to recoup the CapEx investments in building aircraft and the OpEx in operating the fleet. While Arrow may benefit from the simplicity of the business model and enjoy the superior quality control from a vertically integrated business, the upfront capital and talent investments may result in a protracted pre-revenue phase.

More, the OEM space is already crowded with mature players like [Lilium](https://lilium.com/), [Joby](https://www.jobyaviation.com/), [Elroy Air](https://elroyair.com/), [Zipline](https://www.flyzipline.com/), etc., many of them have already had commercial operations ([here](https://www.pfizer.com/news/press-release/press-release-detail/zipline-pfizer-and-biontech-collaboration-paves-way) and [here](https://aerospaceamerica.aiaa.org/elroy-fedex-tests-cargo-delivery/)). The Services team believes that competing in a traditional, CapEx- and operation-heavy business segment is challenging given the long development cycle and the dynamics of commoditization.

Last but not least, certifications with different government agencies around the world can be difficult to navigate and require significant legal resources and even lobbying efforts. Diverting our limited resources and contributors' time on bureaucratic processes may drag down the progress of the entire organization, especially given that Arrow operates on an anonymity-first principle, which can draw extra scrutiny from aviation regulatory bodies.

### Marketplace Platform
An alternative business model would be a marketplace platform like that of Uber. Arrow would solicit bids from OEM players to supply their fleet to our network, and each OEM player would have the discretion to submit a bidding for their vehicles for particular trips.

For example, if a customer wants to ship a 10-kg cargo from San Francisco to San Jose departing April 10th, Arrow would present a list of options. So, the customer may see Elroy, Joby, and Zipline vehicles available for this particular itinerary but at different pricing points:

*(Illustrative purpose only)*
* Joby Cargo: $47.2
* Lilium Cargo: $50.1
* Zipline Cargo: $51.3

All else being equal, the customer would naturally pick the Joby option because it is the cheapest. This behavior will incentivize OEMs to innovate in order to compete with other players on price.

The innovation can come from hardware (advanced manufacturing, superior procurement management, economic design, etc), or from algorithm. Some well-funded players may not care about recouping CapEx, so they would develop algorithms that use Arrow's available APIs to analyze the supply and demand of the market, pricing of other services, etc.

#### Pricing as a Service
Arrow may offer a set of APIs for OEMs to develop in-house algorithms, but it could also be a product offering.

Since Arrow has the full-access to everything on the network, it may makes sense to offer a white-labeled pricing algorithm solution to OEM players, which 1) provides an additional source of revenue to Arrow, and 2) help these OEM players to narrow their focus to the things that they are good at -- making vehicles.

We believe this business model creates a win-win situation where it releases the production and legal burdens from Arrow, and it serves as a direct-to-consumer channel to OEMs, allowing them to monetize on their vehicles with a shorter time-to-market than if they have to sell them through contracts.


#### Downside
While the marketplace business model bridges many gaps on the market among different participants, the platform (Arrow) itself may under-perform as operating a marketplace is difficult and the success is largely a function of [network effects](https://online.hbs.edu/blog/post/what-are-network-effects). 

There essentially three parties in a marketplace ecosystem: the platform (Arrow), the service providers (OEMs or other fleet owner-operators), the customers (passengers, corporations, etc.).

At the early stage, Arrow would have to make outreach efforts to both service providers and customers because a lack of either parties will fail the business. If Arrow were to pursue this business model, a well-thought-out go-to-market strategy is needed and the execution has to be near flawless.

#### Further Discussions
Key points to discuss:
* Whether Arrow wants to pursue a marketplace model
* If we go with the vertical integration model, what kind of operations are we capable of running commercially
* Who and when to showcase our demo
* Should we seek strategic investors
* Alternative source of funds from grants and government subsidies to support our operations. Would this be a sustainable strategy? How much do we need to support us to become cash flow positive? Would there be influences by governments on what we do?


## Risks and Potential Issues
***The underlying pricing model may be incorrectly constructed, thereby over- or under-price trips.***

The pricing logic of `svc-pricing` derives from our [pricing model](https://docs.google.com/spreadsheets/d/1mjPtaIn3E5m7r4nyKt_sJKG9BSFm2ty7Gzo7OqERxwo), which is preliminary and will be submitted for review to external consultants. And currently, the pricing model makes a number of rough estimates on some key inputs like the cost basis of a cargo aircraft. With inaccurate pricing Arrow's business, reputation, results of operations and financial condition can be materially and adversely affected. 

***The pricing service may experience outage.***

Our cloud service providers and other services `svc-pricing` depends on may be impacted by political events, trade and other international disputes, war, terrorism, natural disasters, public health issues, industrial accidents and other business interruptions. 

The unavailability of the pricing service will materially and adversely disrupt the ridesharing network by disabling scheduling of new trips, which may result in an increased number of underutilized aircraft, vertiports, pilots, and staffs. Our customer base may experience churns as a result of unavailable services. Additional costs may be incurred to fix the outage and to resume the network activity.

## Appendix A: Citations
TODO
## Appendix B: Acronyms & Glossary
See the [Arrow Glossary](https://www.arrowair.com/docs/documentation/glossary).
