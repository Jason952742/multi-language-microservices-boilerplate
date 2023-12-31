plugins {
    kotlin("jvm") version "1.9.21"
    kotlin("plugin.allopen") version "1.9.21"
    kotlin("plugin.noarg") version "1.9.21"
    kotlin("plugin.serialization") version "1.9.21"

    id("io.quarkus")
    id("java")
}

repositories {
    mavenCentral()
    mavenLocal()
    jcenter()
}

val quarkusPlatformGroupId: String by project
val quarkusPlatformArtifactId: String by project
val quarkusPlatformVersion: String by project

dependencies {
    implementation(enforcedPlatform("${quarkusPlatformGroupId}:${quarkusPlatformArtifactId}:${quarkusPlatformVersion}"))

    implementation("io.quarkus:quarkus-kotlin:3.6.4.Final")
    implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk8:1.8.10")
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.5.0")
    implementation("io.smallrye.reactive:mutiny-kotlin:2.1.0")

    implementation("io.quarkus:quarkus-resteasy-reactive:3.6.4.Final")
    implementation("io.quarkus:quarkus-resteasy-reactive-qute:3.6.4.Final")
    implementation("io.quarkus:quarkus-resteasy-reactive-jackson:3.6.4.Final")
    implementation("io.quarkus:quarkus-resteasy-reactive-kotlin-serialization:3.6.4.Final")

    implementation("io.quarkus:quarkus-rest-client-reactive-jackson:3.6.4.Final")
    implementation("io.quarkus:quarkus-rest-client-reactive-kotlin-serialization:3.6.4.Final")

    implementation("io.quarkus:quarkus-hibernate-envers:3.6.4.Final")
    implementation("io.quarkus:quarkus-hibernate-reactive-panache-kotlin:3.6.4.Final")
    implementation("io.quarkus:quarkus-reactive-pg-client:3.6.4.Final")
    implementation("io.quarkus:quarkus-hibernate-validator:3.6.4.Final")

    implementation("io.quarkus:quarkus-smallrye-openapi:3.6.4.Final")
    implementation("io.quarkus:quarkus-smallrye-health:3.6.4.Final")
    implementation("io.quarkus:quarkus-smallrye-context-propagation:3.6.4.Final")
    implementation("io.quarkus:quarkus-smallrye-reactive-messaging:3.6.4.Final")
    implementation("io.quarkus:quarkus-smallrye-reactive-messaging-rabbitmq:3.6.4.Final")
    implementation("io.quarkus:quarkus-smallrye-jwt:3.6.4.Final")
    implementation("io.quarkus:quarkus-smallrye-jwt-build:3.6.4.Final")
    implementation("io.quarkus:quarkus-smallrye-fault-tolerance:3.6.4.Final")

    implementation("io.quarkus:quarkus-redis-client:3.6.4.Final")
    implementation("io.quarkus:quarkus-redis-cache:3.6.4.Final")
    implementation("io.quarkus:quarkus-cache:3.6.4.Final")
    implementation("io.quarkiverse.neo4j:quarkus-neo4j:3.5.0")

    implementation("io.quarkus:quarkus-websockets:3.6.4.Final")
    implementation("io.quarkus:quarkus-websockets-client:3.6.4.Final")

    implementation("io.quarkiverse.openapi.generator:quarkus-openapi-generator:2.2.14")
    implementation("io.quarkus:quarkus-mutiny:3.6.4.Final")
    implementation("io.quarkus:quarkus-grpc:3.6.4.Final")
    implementation("io.quarkus:quarkus-quartz:3.6.4.Final")
    implementation("io.quarkus:quarkus-scheduler:3.6.4.Final")

    implementation("io.quarkus:quarkus-jackson:3.6.4.Final")
    implementation("io.quarkus:quarkus-scheduler:3.6.4.Final")
    implementation("io.quarkus:quarkus-arc:3.6.4.Final")
    implementation("commons-codec:commons-codec:1.15")
    implementation("io.github.novacrypto:BIP39:2019.01.27")
    implementation("com.ecwid.consul:consul-api:1.4.5")

    testImplementation("io.quarkus:quarkus-junit5:3.6.4.Final")
    testImplementation("io.rest-assured:rest-assured:5.3.0")
    testImplementation("org.awaitility:awaitility:4.2.0")
    testImplementation("org.assertj:assertj-core:3.24.2")
    testImplementation("io.rest-assured:kotlin-extensions:5.3.0")
}

group = "org.acme"
version = "1.0.0-SNAPSHOT"

java {
    sourceCompatibility = JavaVersion.VERSION_20
    targetCompatibility = JavaVersion.VERSION_20
}

tasks.withType<Test> {
    systemProperty("java.util.logging.manager", "org.jboss.logmanager.LogManager")
}

noArg {
    annotation("jakarta.persistence.Entity")
    annotation("jakarta.persistence.Embeddable")
    annotation("jakarta.persistence.MappedSuperclass")
}

allOpen {
    annotation("jakarta.ws.rs.Path")
    annotation("jakarta.enterprise.context.ApplicationScoped")
    annotation("jakarta.persistence.Entity")
    annotation("io.quarkus.test.junit.QuarkusTest")
    annotation("jakarta.persistence.MappedSuperclass")
    annotation("jakarta.persistence.Embeddable")
}

tasks.withType<org.jetbrains.kotlin.gradle.tasks.KotlinCompile> {
    kotlinOptions.jvmTarget = JavaVersion.VERSION_20.toString()
    kotlinOptions.javaParameters = true
}
