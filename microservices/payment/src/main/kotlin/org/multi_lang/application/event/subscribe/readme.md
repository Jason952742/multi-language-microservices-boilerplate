# Event Subscription

Storing event subscription related code. In order to achieve the unified management of event subscription, all event subscription related code within the microservice is unified into the application layer. The core business logic implementation of event processing can be placed in the domain layer. The application layer calls the domain service to achieve the complete event subscription process.
