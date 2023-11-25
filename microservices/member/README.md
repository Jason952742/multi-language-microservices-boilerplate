# member microservice

This project uses Quarkus, the Supersonic Subatomic Java Framework.

If you want to learn more about Quarkus, please visit its website: https://quarkus.io/ .

## Running the application in dev mode

You can run your application in dev mode that enables live coding using:
```shell script
./gradlew quarkusDev
```

> **_NOTE:_**  Quarkus now ships with a Dev UI, which is available in dev mode only at http://localhost:8080/q/dev/.

## Packaging and running the application

The application can be packaged using:
```shell script
./gradlew build
```
It produces the `quarkus-run.jar` file in the `build/quarkus-app/` directory.
Be aware that it’s not an _über-jar_ as the dependencies are copied into the `build/quarkus-app/lib/` directory.

The application is now runnable using `java -jar build/quarkus-app/quarkus-run.jar`.

If you want to build an _über-jar_, execute the following command:
```shell script
./gradlew build -Dquarkus.package.type=uber-jar
```

The application, packaged as an _über-jar_, is now runnable using `java -jar build/*-runner.jar`.

## Creating a native executable

You can create a native executable using: 
```shell script
./gradlew build -Dquarkus.package.type=native
```

Or, if you don't have GraalVM installed, you can run the native executable build in a container using: 
```shell script
./gradlew build -Dquarkus.package.type=native -Dquarkus.native.container-build=true
```

You can then execute your native executable with: `./build/code-with-quarkus-1.0.0-SNAPSHOT-runner`

If you want to learn more about building native executables, please consult https://quarkus.io/guides/gradle-tooling.

## Related Guides

- SmallRye GraphQL ([guide](https://quarkus.io/guides/smallrye-graphql)): Create GraphQL Endpoints using the code-first approach from MicroProfile GraphQL
- JDBC Driver - PostgreSQL ([guide](https://quarkus.io/guides/datasource)): Connect to the PostgreSQL database via JDBC
- Keycloak Authorization ([guide](https://quarkus.io/guides/security-keycloak-authorization)): Policy enforcer using Keycloak-managed permissions to control access to protected resources
- RESTEasy Classic Mutiny ([guide](https://quarkus.io/guides/resteasy#reactive)): Mutiny support for RESTEasy Classic server
- SmallRye OpenAPI ([guide](https://quarkus.io/guides/openapi-swaggerui)): Document your REST APIs with OpenAPI - comes with Swagger UI
- SmallRye GraphQL Client ([guide](https://quarkus.io/guides/smallrye-graphql-client)): Create GraphQL Clients
- Kotlin ([guide](https://quarkus.io/guides/kotlin)): Write your services in Kotlin
- Hibernate Search + Elasticsearch ([guide](https://quarkus.io/guides/hibernate-search-orm-elasticsearch)): Automatically index your Hibernate entities in Elasticsearch
- Hibernate Envers ([guide](https://quarkus.io/guides/hibernate-orm#envers)): Enable Hibernate Envers capabilities in your Jakarta Persistence applications
- Mutiny ([guide](https://quarkus.io/guides/mutiny-primer)): Write reactive applications with the modern Reactive Programming library Mutiny
- Hibernate ORM with Panache and Kotlin ([guide](https://quarkus.io/guides/hibernate-orm-panache-kotlin)): Define your persistent model in Hibernate ORM with Panache
- RESTEasy Reactive ([guide](https://quarkus.io/guides/resteasy-reactive)): A Jakarta REST implementation utilizing build time processing and Vert.x. This extension is not compatible with the quarkus-resteasy extension, or any of the extensions that depend on it.
- RESTEasy Classic Multipart ([guide](https://quarkus.io/guides/rest-json#multipart-support)): Multipart support for RESTEasy Classic
- YAML Configuration ([guide](https://quarkus.io/guides/config-yaml)): Use YAML to configure your Quarkus application
- SmallRye Health ([guide](https://quarkus.io/guides/smallrye-health)): Monitor service health
- SmallRye Context Propagation ([guide](https://quarkus.io/guides/context-propagation)): Propagate contexts between managed threads in reactive applications
- SmallRye Reactive Messaging - AMQP Connector ([guide](https://quarkus.io/guides/amqp)): Connect to AMQP with Reactive Messaging
- Eclipse Vert.x ([guide](https://quarkus.io/guides/vertx)): Write reactive applications with the Vert.x API
- WebSockets Client ([guide](https://quarkus.io/guides/websockets)): Client for WebSocket communication channel
- Hibernate Validator ([guide](https://quarkus.io/guides/validation)): Validate object properties (field, getter) and method parameters for your beans (REST, CDI, Jakarta Persistence)
- Quartz ([guide](https://quarkus.io/guides/quartz)): Schedule clustered tasks with Quartz
- Neo4j client ([guide](https://quarkiverse.github.io/quarkiverse-docs/quarkus-neo4j/dev/index.html)): Connect to Neo4j graph datastore
- WebSockets ([guide](https://quarkus.io/guides/websockets)): WebSocket communication channel support
- Redis Client ([guide](https://quarkus.io/guides/redis)): Connect to Redis in either imperative or reactive style
- Cache ([guide](https://quarkus.io/guides/cache)): Enable application data caching in CDI beans

## Provided Code

### YAML Config

Configure your application with YAML

[Related guide section...](https://quarkus.io/guides/config-reference#configuration-examples)

The Quarkus application configuration is located in `src/main/resources/application.yml`.

### Hibernate ORM

Create your first JPA entity

[Related guide section...](https://quarkus.io/guides/hibernate-orm)


[Related Hibernate with Panache in Kotlin section...](https://quarkus.io/guides/hibernate-orm-panache-kotlin)

### RESTEasy JAX-RS

Easily start your RESTful Web Services

[Related guide section...](https://quarkus.io/guides/getting-started#the-jax-rs-resources)

### RESTEasy Reactive

Easily start your Reactive RESTful Web Services

[Related guide section...](https://quarkus.io/guides/getting-started-reactive#reactive-jax-rs-resources)

### RESTEasy Reactive Qute

Create your web page using Quarkus RESTEasy Reactive & Qute

[Related guide section...](https://quarkus.io/guides/qute#type-safe-templates)

### SmallRye GraphQL

Start coding with this Hello GraphQL Query

[Related guide section...](https://quarkus.io/guides/smallrye-graphql)

### SmallRye Health

Monitor your application's health using SmallRye Health

[Related guide section...](https://quarkus.io/guides/smallrye-health)

### WebSockets

WebSocket communication channel starter code

[Related guide section...](https://quarkus.io/guides/websockets)
