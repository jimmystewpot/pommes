use std::fs::read_to_string;

use askama::Template;
use structopt::StructOpt;

use pomm::simpler::Project;

#[derive(Debug, StructOpt)]
struct CliOptions {
    modules: Vec<String>,
    #[structopt(long, short = "t")]
    without_tests: bool,
}

#[derive(Template)]
#[template(path = "package.spec", escape = "none")]
struct SpecTemplate {
    name: String,
    version: String,
    summary: String,
    license: String,
    url: String,
    source: String,
    description: String,
    modules: Vec<String>,
    with_tests: bool,
}

fn main() -> Result<(), String> {
    let args: CliOptions = CliOptions::from_args();

    let string = match read_to_string("pom.xml") {
        Ok(result) => result,
        Err(error) => return Err(error.to_string()),
    };

    let project: pomm::parser::Project = match serde_xml_rs::from_str(&string) {
        Ok(result) => result,
        Err(error) => return Err(error.to_string()),
    };

    let project: Project = project.into();

    let template = SpecTemplate {
        name: match &project.name {
            Some(name) => name.to_owned(),
            None => project.artifact_id.replace(",", "."),
        },
        version: match &project.version {
            Some(version) => version.replace("-", "~"),
            None => "FILL ME!".to_owned(),
        },
        summary: "FILL ME!".to_owned(),
        license: match &project.licenses.len() {
            0 => "FILL ME!".to_owned(),
            _ => project.licenses.iter().map(|l| l.name.clone()).collect::<Vec<String>>().join(" and "),
        },
        url: match &project.url {
            Some(url) => url.to_owned(),
            None => "FILL ME!".to_owned(),
        },
        source: "FILL ME!".to_owned(),
        description: match &project.description {
            Some(description) => textwrap::fill(description, 72),
            None => "FILL ME!".to_owned(),
        },
        modules: args.modules,
        with_tests: !args.without_tests,
    };

    println!("{}", template.render().unwrap());

    Ok(())
}
