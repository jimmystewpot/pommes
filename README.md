# project object model model (and parser)

This project contains a model definition of maven POM files, with it an easy way to parse `pom.xml` files into data
structures using `serde` / `serde-xml-rs`.

There's also a `mvn-genbr` binary that can be used to generate `BuildRequires` for RPM builds.

