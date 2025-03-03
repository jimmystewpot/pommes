# Project object model (POM) model and parser (DEPRECATED) **FORK**

**WARNING**: This project is no longer actively developed.

This project contains a model definition of maven POM files, with it an easy way
to parse `pom.xml` files into data structures using `serde` / `serde-xml-rs`.

## mvn-genbr

The `mvn-genbr` binary can be used to generate `BuildRequires` for RPM builds.

It's now available in `rawhide` as an experimental feature. To use it for your
package, `BuildRequires` for `maven-local` (as usual) and `mvn-genbr` are
necessary. 

Do your usual processing in `%prep` (all `pom.xml` modifications, like modifying
dependencies and enabling/disabling plugins). Then, to use automatically generated
`BuildRequires`:

```
%generate_buildrequires
mvn-genbr .
```

To enable dependencies for tests (with the `test` scope in maven), use the `-t`
parameter. To use additional (or different) root directories (instead of `.`),
just add those as arguments. The program will look for `pom.xml` files in those
paths, parse them, and print the corresponding `BuildRequires` in the format
that `rpm` / `mock` expect them.

