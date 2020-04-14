use std::fs::read_to_string;
use std::path::PathBuf;

use glob::glob;

use pomm;

#[test]
fn all() {
    for found in glob("./tests/**/pom.xml").unwrap() {
        let path: PathBuf = found.unwrap();
        println!("path: {}", &path.display());

        let contents = read_to_string(path).unwrap();
        let parsed: pomm::parser::Project = serde_xml_rs::from_str(&contents).unwrap();
        let _project: pomm::simpler::Project = parsed.into();
        //println!("{:#?}", project);
    }
}
