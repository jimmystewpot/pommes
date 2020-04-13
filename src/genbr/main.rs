use std::fs;
use std::path::PathBuf;

use structopt::StructOpt;

use pomm::Project;

#[derive(Debug, StructOpt)]
struct CliOptions {
    modules: Vec<String>,
    #[structopt(long, short = "t")]
    with_tests: bool,
}

fn main() -> Result<(), String> {
    let args: CliOptions = CliOptions::from_args();

    let mut modules: Vec<String> = args.modules.iter().map(|s| s.to_string()).collect();

    let mut provided: Vec<String> = Vec::new();
    let mut processed: Vec<String> = Vec::new();
    let mut mod_error: Vec<(String, String)> = Vec::new();

    let mut dependencies: Vec<String> = Vec::new();

    while !modules.is_empty() {
        let name = modules.remove(0);

        if processed.contains(&name) {
            continue;
        };

        let path = PathBuf::from(".").join(&name).join("pom.xml");

        let contents = match fs::read_to_string(path.as_path()) {
            Ok(contents) => contents,
            Err(error) => {
                mod_error.push((name, error.to_string()));
                continue;
            },
        };

        let project: Project = match serde_xml_rs::from_str(&contents) {
            Ok(project) => project,
            Err(error) => {
                mod_error.push((name, error.to_string()));
                continue;
            },
        };

        match (&project.group_id, &project.parent) {
            (Some(group_id), _) => provided.push(format!("{}:{}", group_id, &project.artifact_id)),
            (None, Some(parent)) => provided.push(format!("{}:{}", &parent.group_id, &project.artifact_id)),
            (None, None) => {
                mod_error.push((name, String::from("Unable to determine groupId.")));
                continue;
            },
        };

        if let Some(parent) = &project.parent {
            dependencies.push(format!("{}:{}", &parent.group_id, &parent.artifact_id));
        }

        if let Some(deps) = &project.dependencies {
            for dep in &deps.dependencies {
                if let Some(scope) = &dep.scope {
                    if scope == "test" && !args.with_tests {
                        continue;
                    };
                };

                dependencies.push(format!("{}:{}", &dep.group_id, &dep.artifact_id));
            }
        };

        if let Some(submodules) = &project.modules {
            for module in &submodules.modules {
                modules.push(format!("{}/{}", name, module));
            }
        };

        processed.push(name);
    }

    if !mod_error.is_empty() {
        for (module, error) in mod_error {
            eprintln!("Failed to process module {}: {}", module, error);
        }

        return Err(String::from("Failed to parse some POM files."));
    };

    dependencies.sort();
    dependencies.dedup_by(|a, b| a == b);

    for dep in dependencies {
        println!("mvn({})", dep);
    }

    Ok(())
}
