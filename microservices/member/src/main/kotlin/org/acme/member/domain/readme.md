# Domain layer

Holds code related to the core business logic of the domain layer. The domain layer can contain multiple aggregation code packages that together implement the core business logic of the domain model. The aggregation roots within the aggregation and related code such as entities, methods, value objects, domain services and events are placed in this layer of directories.

The directory structure below the domain layer is composed of one or more independent aggregate directories, each aggregate is an independent business functional unit, and multiple aggregates work together to implement the core business logic of the domain model.
