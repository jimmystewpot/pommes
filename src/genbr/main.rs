use std::fs;
use std::path::PathBuf;

use structopt::StructOpt;

use pommes::Project;

#[derive(Debug, StructOpt)]
struct CliOptions {
    /// Module root paths to consider for dependency resolution
    modules: Vec<String>,
    /// Include dependencies with "test" scope
    #[structopt(long, short = "t")]
    with_tests: bool,
    /// Additionally write dependencies to '.mvn-genbr' file
    #[structopt(long, short = "w")]
    write_to_file: bool,
}

fn main() -> Result<(), String> {
    let args: CliOptions = CliOptions::from_args();

    let mut modules: Vec<String> = args.modules.iter().map(|s| s.to_string()).collect();
    let mut processed: Vec<String> = Vec::new();

    let mut provided: Vec<(String, String)> = Vec::new();
    let mut dependencies: Vec<(String, String)> = Vec::new();

    let mut mod_error: Vec<(String, String)> = Vec::new();

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
            (Some(group_id), _) => provided.push((group_id.to_string(), project.artifact_id.to_string())),
            (None, Some(parent)) => provided.push((parent.group_id.to_string(), project.artifact_id.to_string())),
            (None, None) => {
                mod_error.push((name, String::from("Unable to determine groupId.")));
                continue;
            },
        };

        let mut _active_profile: Option<String> = None;
        let mut profile_modules: Vec<String> = Vec::new();

        if let Some(profiles) = &project.profiles {
            for profile in &profiles.profiles {
                if let Some(activation) = &profile.activation {
                    if let Some(true) = activation.active_by_default {
                        if let Some(id) = &profile.id {
                            _active_profile = Some(id.to_owned());
                        }

                        if let Some(modules) = &profile.modules {
                            profile_modules.extend(modules.modules.clone());
                        }
                    };
                };
            }
        }

        if let Some(parent) = &project.parent {
            dependencies.push((parent.group_id.to_string(), parent.artifact_id.to_string()));
        }

        if let Some(deps) = &project.dependencies {
            for dep in &deps.dependencies {
                if let Some(scope) = &dep.scope {
                    if scope == "test" && !args.with_tests {
                        continue;
                    };
                };

                dependencies.push((dep.group_id.to_string(), dep.artifact_id.to_string()));
            }
        }

        if let Some(build) = &project.build {
            if let Some(plugins) = &build.plugins {
                for plugin in &plugins.plugins {
                    dependencies.push((plugin.group_id.to_string(), plugin.artifact_id.to_string()));
                }
            }
        }

        if let Some(mods) = &project.modules {
            for module in &mods.modules {
                modules.push(format!("{}/{}", name, module));
            }
        }

        modules.extend(profile_modules);

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

    let mut output = String::new();

    for dep in dependencies {
        if !provided.contains(&dep) {
            output.push_str(&format!("mvn({}:{})\n", dep.0, dep.1));
        }
    }

    print!("{}", &output);

    if args.write_to_file && fs::write(".mvn-genbr", &output).is_err() {
        eprintln!("Failed to write results to file '.mvn-genbr'.");
    }

    Ok(())
}
