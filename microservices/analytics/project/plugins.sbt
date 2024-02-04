resolvers += "Akka library repository".at("https://repo.akka.io/maven")

addSbtPlugin("com.lightbend.akka.grpc" % "sbt-akka-grpc" % "2.4.0")

addSbtPlugin("com.lightbend.sbt" % "sbt-javaagent" % "0.1.5")
