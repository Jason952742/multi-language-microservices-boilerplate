name := "analytics"

version := "1.0"

scalaVersion := "3.3.1"

lazy val akkaVersion = "2.9.0"
lazy val akkaGrpcVersion = "2.4.0"

enablePlugins(AkkaGrpcPlugin)

// Run in a separate JVM, to make sure sbt waits until all threads have
// finished before returning.
// If you want to keep the application running while executing other
// sbt tasks, consider https://github.com/spray/sbt-revolver/
fork := true

resolvers ++= Seq(
  "Akka library repository".at("https://repo.akka.io/maven"),
  // "Sonatype Snapshots" at "https://oss.sonatype.org/content/repositories/snapshots/"
)

libraryDependencies ++= Seq(
  "com.typesafe.akka" %% "akka-actor-typed" % akkaVersion,
  "com.typesafe.akka" %% "akka-stream" % akkaVersion,
  "com.typesafe.akka" %% "akka-discovery" % akkaVersion,
  "com.typesafe.akka" %% "akka-pki" % akkaVersion,

  "ch.qos.logback" % "logback-classic" % "1.4.12",
  "com.orbitz.consul" % "consul-client" % "1.5.3",

  "com.typesafe.akka" %% "akka-actor-testkit-typed" % akkaVersion % Test,
  "com.typesafe.akka" %% "akka-stream-testkit" % akkaVersion % Test,
  "org.scalatest" %% "scalatest" % "3.2.15" % Test
)
