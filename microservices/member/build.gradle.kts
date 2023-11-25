plugins {
    kotlin("jvm") version "1.9.10"
    kotlin("plugin.allopen") version "1.9.10"
    id("io.quarkus")
}

repositories {
    mavenCentral()
    mavenLocal()
}

val quarkusPlatformGroupId: String by project
val quarkusPlatformArtifactId: String by project
val quarkusPlatformVersion: String by project

dependencies {
    implementation(enforcedPlatform("${quarkusPlatformGroupId}:${quarkusPlatformArtifactId}:${quarkusPlatformVersion}"))
    implementation("io.quarkus:quarkus-rest-client-reactive-kotlin-serialization")
    implementation("io.quarkus:quarkus-smallrye-graphql")
    implementation("io.quarkus:quarkus-rest-client-mutiny")
    implementation("io.quarkus:quarkus-rest-client-reactive-jackson")
    implementation("io.quarkus:quarkus-jdbc-postgresql")
    implementation("io.quarkus:quarkus-keycloak-authorization")
    implementation("io.quarkus:quarkus-resteasy-mutiny")
    implementation("io.quarkus:quarkus-smallrye-openapi")
    implementation("io.quarkus:quarkus-smallrye-graphql-client")
    implementation("io.quarkus:quarkus-kotlin")
    implementation("io.quarkus:quarkus-hibernate-search-orm-elasticsearch")
    implementation("io.quarkus:quarkus-hibernate-envers")
    implementation("io.quarkus:quarkus-keycloak-admin-client-reactive")
    implementation("io.quarkus:quarkus-resteasy-reactive-qute")
    implementation("io.quarkus:quarkus-mutiny")
    implementation("io.quarkus:quarkus-resteasy-reactive-jackson")
    implementation("io.quarkus:quarkus-hibernate-orm-panache-kotlin")
    implementation("io.quarkus:quarkus-resteasy-reactive")
    implementation("io.quarkus:quarkus-resteasy-multipart")
    implementation("io.quarkus:quarkus-config-yaml")
    implementation("io.quarkus:quarkus-smallrye-health")
    implementation("io.quarkus:quarkus-smallrye-context-propagation")
    implementation("io.quarkus:quarkus-resteasy-reactive-kotlin-serialization")
    implementation("io.quarkus:quarkus-grpc")
    implementation("io.quarkus:quarkus-smallrye-reactive-messaging-amqp")
    implementation("io.quarkus:quarkus-vertx")
    implementation("io.quarkus:quarkus-websockets-client")
    implementation("io.quarkus:quarkus-hibernate-validator")
    implementation("io.quarkus:quarkus-quartz")
    implementation("io.quarkiverse.neo4j:quarkus-neo4j:3.5.0")
    implementation("io.quarkus:quarkus-websockets")
    implementation("io.quarkus:quarkus-jackson")
    implementation("io.quarkus:quarkus-redis-client")
    implementation("io.quarkus:quarkus-cache")
    implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk8")
    implementation("io.quarkus:quarkus-arc")
    implementation("io.quarkus:quarkus-hibernate-orm")
    implementation("io.quarkus:quarkus-resteasy")
    testImplementation("io.quarkus:quarkus-junit5")
    testImplementation("io.rest-assured:rest-assured")
}

group = "org.acme"
version = "1.0.0-SNAPSHOT"

java {
    sourceCompatibility = JavaVersion.VERSION_17
    targetCompatibility = JavaVersion.VERSION_17
}

tasks.withType<Test> {
    systemProperty("java.util.logging.manager", "org.jboss.logmanager.LogManager")
}
allOpen {
    annotation("jakarta.ws.rs.Path")
    annotation("jakarta.enterprise.context.ApplicationScoped")
    annotation("jakarta.persistence.Entity")
    annotation("io.quarkus.test.junit.QuarkusTest")
}

tasks.withType<org.jetbrains.kotlin.gradle.tasks.KotlinCompile> {
    kotlinOptions.jvmTarget = JavaVersion.VERSION_17.toString()
    kotlinOptions.javaParameters = true
}
