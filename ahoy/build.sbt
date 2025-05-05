ThisBuild / scalaVersion := "3.6.4"

lazy val root = (project in file("."))
  .settings(
    name := "ahoy",
    version := "0.1.0",
    libraryDependencies ++= Seq(
      "com.typesafe.akka" %% "akka-http" % "10.5.2",
      "com.typesafe.akka" %% "akka-stream" % "2.8.5",
      "de.heikoseeberger" %% "akka-http-circe" % "1.40.0",
      "io.circe" %% "circe-generic" % "0.14.6",
      "org.tpolecat" %% "doobie-core" % "1.0.0-RC4",
      "org.tpolecat" %% "doobie-postgres" % "1.0.0-RC4",
      "org.mindrot" % "jbcrypt" % "0.4"
    )
  )

