# Application Service

Application services will encapsulate, orchestrate and combine application services from multiple domain services or other microservices to provide coarse-grained services externally. An application service class can be designed for each aggregated application service.
In addition, when making cross-microservice calls, some DO objects need to be converted into DTOs, so the application layer may also have assembler and dto objects at the user interface layer. At this point, the assembler and dto code directory structure can be added as needed.

For complex queries with multi-table associations, it is not recommended to put such complex queries in the domain model of the domain layer because such complex queries do not need to be constrained by domain logic and business rules.
You can use the traditional multi-table related SQL query through the application services in the application layer, or you can use the CQRS read-write separation to complete the data query operation.

For example, using the CQRS read-write separation approach, view the SEARCH catalogue.
