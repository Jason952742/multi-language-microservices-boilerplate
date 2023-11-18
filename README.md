# Multi-Lang Microservices Boilerplate
Example of how to build a simple microservices architecture using KrakenD + Keycloak + Nacos + Seata + Sentinel and multiple programming languages

## Features
- High availability / High performance / Scaling
- Production grade
- Fastest API Gateway (Proxy, Security, Cache, QoS, Aggregation, Monitoring, Throttling, Decoding, etc.)
- IdP Authorization Services (OIDC/Social Login/SSO)
- Dynamic Configuration Service
- Service Discovery and Management
- Dynamic DNS Service
- Distributed transaction solution (AT/TCC/XA/SAGA mode)
- Flow Control / Circuit Breaking and Concurrency / Adaptive System Protection
- Multiple programming languages example (e.g., Rust/Kotlin/Scala/Java/C/Python/Golang/NodeJS/.NET)
- Distributed Tracing / Consistent Metrics Aggregation / Log Management / Alerting and Telemetry
- DDD(Domain-driven design), CQRS/ES and Actor Model
- RESTFul API, GraphQL API, GRPC and AMQP (Nats/RabbitMQ)

## Precondition
Make sure you have Docker and Node installed:  
- [NodeJS](https://nodejs.org/) 
- [Docker](https://www.docker.com)

## Usage

### Step 1: Keycloak Deployment and Configuration
create a network：
```bash
docker network create multi-lang-network --driver bridge
```
and Refer to [Keycloak Production Development](keycloak/README.md)

## License

Licensed under of

- MIT license ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT)
  at your option.