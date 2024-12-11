val scala3Version = "3.6.2"

organizationName := "Nigel Eke"
organization     := "nigeleke"
startYear        := Some(2022)
licenses += ("BSD-3-Clause", new URL("https://opensource.org/licenses/BSD-3-Clause"))

val configVersion        = "1.4.3"
val scalatestVersion     = "3.2.19"
val scalatestPlusVersion = scalatestVersion + ".0"

ThisBuild / versionScheme := Some("early-semver")
ThisBuild / publishTo     := Some(Resolver.defaultLocal)

lazy val root = project
  .in(file("."))
  .settings(
    name         := "life",
    scalaVersion := scala3Version,
    libraryDependencies ++= Seq(
      "com.typesafe"       % "config"          % configVersion,
      "org.scalactic"     %% "scalactic"       % scalatestVersion,
      "org.scalatest"     %% "scalatest"       % scalatestVersion     % "test",
      "org.scalatestplus" %% "scalacheck-1-18" % scalatestPlusVersion % "test"
    )
  )
