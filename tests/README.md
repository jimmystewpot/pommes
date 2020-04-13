# doing real-world tests

Place `pom.xml` files somewhere in this directory (e.g. exploded tarballs or git
checkouts of projects using maven), and run `cargo test`, which will find all
`pom.xml` files and try to parse them. If this doesn't work, you've found a bug!
