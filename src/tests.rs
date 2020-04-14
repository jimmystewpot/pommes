use crate::parser::*;

#[test]
fn de_project() {
    let string = r#"
        <project xmlns="http://maven.apache.org/POM/4.0.0">
            <modelVersion>4.0.0</modelVersion>
            <groupId>io.pagure.java</groupId>
            <artifactId>maven.rs</artifactId>
            <version>0.0.0</version>
            <parent>
                <groupId>io.pagure</groupId>
                <artifactId>java</artifactId>
            </parent>
            <packaging>jar</packaging>
            <dependencies>
                <dependency>
                    <groupId>junit</groupId>
                    <artifactId>junit</artifactId>
                    <version>4.12</version>
                    <type>jar</type>
                    <scope>test</scope>
                    <optional>true</optional>
                </dependency>
            </dependencies>
            <modules>
                <module>test</module>
            </modules>
        </project>
        "#;

    let expected = Project {
        model_version: String::from("4.0.0"),
        group_id: Some(String::from("io.pagure.java")),
        artifact_id: String::from("maven.rs"),
        version: Some(String::from("0.0.0")),
        packaging: Some(String::from("jar")),
        parent: Some(Parent {
            group_id: String::from("io.pagure"),
            artifact_id: String::from("java"),
            version: None,
        }),
        dependencies: Some(Dependencies {
            dependencies: vec![Dependency {
                group_id: String::from("junit"),
                artifact_id: String::from("junit"),
                version: Some(String::from("4.12")),
                dtype: Some(String::from("jar")),
                scope: Some(String::from("test")),
                optional: Some(true),
            }],
        }),
        modules: Some(Modules {
            modules: vec![String::from("test")],
        }), ..Default::default()
    };

    let parsed: Project = serde_xml_rs::from_str(string).unwrap();

    assert_eq!(parsed, expected);
}
