use std::collections::HashMap;

use crate::parser;

#[derive(Debug)]
pub struct Project {
    pub artifact_id: String,
    pub build: Option<Build>,
    pub contributors: Vec<Person>,
    pub dependencies: Vec<Dependency>,
    pub dependency_management: Vec<Dependency>,
    pub description: Option<String>,
    pub developers: Vec<Person>,
    pub group_id: Option<String>,
    pub inception_year: Option<String>,
    pub licenses: Vec<License>,
    pub model_version: String,
    pub modules: Vec<String>,
    pub name: Option<String>,
    pub organization: Option<Organization>,
    pub packaging: Option<String>,
    pub parent: Option<Parent>,
    pub plugin_repositories: Vec<Repository>,
    pub profiles: Vec<Profile>,
    pub properties: HashMap<String, String>,
    pub repositories: Vec<Repository>,
    pub url: Option<String>,
    pub version: Option<String>,
}

impl From<parser::Project> for Project {
    fn from(project: parser::Project) -> Self {
        Project {
            artifact_id: project.artifact_id,
            build: project.build.map(|b| b.into()),
            contributors: match project.contributors {
                Some(cs) => cs.contributors.into_iter().map(|c| c.into()).collect(),
                None => Vec::new(),
            },
            dependencies: match project.dependencies {
                Some(ds) => ds.dependencies.into_iter().map(|d| d.into()).collect(),
                None => Vec::new(),
            },
            dependency_management: match project.dependency_management {
                Some(ds) => ds.dependencies.into_iter().map(|d| d.into()).collect(),
                None => Vec::new(),
            },
            description: project.description,
            developers: match project.developers {
                Some(ds) => ds.developers.into_iter().map(|d| d.into()).collect(),
                None => Vec::new(),
            },
            group_id: project.group_id,
            inception_year: project.inception_year,
            licenses: match project.licenses {
                Some(ls) => ls.licenses.into_iter().map(|l| l.into()).collect(),
                None => Vec::new(),
            },
            model_version: project.model_version,
            modules: match project.modules {
                Some(ms) => ms.modules.into(),
                None => Vec::new(),
            },
            name: project.name,
            organization: project.organization.map(|o| o.into()),
            packaging: project.packaging,
            parent: project.parent.map(|p| p.into()),
            plugin_repositories: match project.plugin_repositories {
                Some(rs) => rs.repositories.into_iter().map(|p| p.into()).collect(),
                None => Vec::new(),
            },
            profiles: match project.profiles {
                Some(ps) => ps.profiles.into_iter().map(|p| p.into()).collect(),
                None => Vec::new(),
            },
            properties: project.properties,
            repositories: match project.repositories {
                Some(rs) => rs.repositories.into_iter().map(|r| r.into()).collect(),
                None => Vec::new(),
            },
            url: project.url,
            version: project.version,
        }
    }
}

#[derive(Debug)]
pub struct License {
    pub comments: Option<String>,
    pub distribution: Option<String>,
    pub name: String,
    pub url: Option<String>,
}

impl From<parser::License> for License {
    fn from(license: parser::License) -> Self {
        License {
            comments: license.comments,
            distribution: license.distribution,
            name: license.name,
            url: license.url,
        }
    }
}

#[derive(Debug)]
pub struct Organization {
    pub name: Option<String>,
    pub url: Option<String>,
}

impl From<parser::Organization> for Organization {
    fn from(org: parser::Organization) -> Self {
        Organization {
            name: org.name,
            url: org.url,
        }
    }
}

#[derive(Debug)]
pub struct Person {
    pub email: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub organization: Option<String>,
    pub organization_url: Option<String>,
    pub properties: HashMap<String, String>,
    pub roles: Vec<String>,
    pub timezone: Option<String>,
    pub url: Option<String>,
}

impl From<parser::Person> for Person {
    fn from(person: parser::Person) -> Self {
        Person {
            email: person.email,
            id: person.id,
            name: person.name,
            organization: person.organization,
            organization_url: person.organization_url,
            properties: person.properties,
            roles: match person.roles {
                Some(rs) => rs.roles.into(),
                None => Vec::new(),
            },
            timezone: person.timezone,
            url: person.url,
        }
    }
}

#[derive(Debug)]
pub struct Parent {
    pub artifact_id: String,
    pub group_id: String,
    pub version: Option<String>,
}

impl From<parser::Parent> for Parent {
    fn from(parent: parser::Parent) -> Self {
        Parent {
            artifact_id: parent.artifact_id,
            group_id: parent.group_id,
            version: parent.version,
        }
    }
}

#[derive(Debug)]
pub struct Dependency {
    pub artifact_id: String,
    pub dtype: Option<String>,
    pub group_id: String,
    pub optional: Option<bool>,
    pub scope: Option<String>,
    pub version: Option<String>,
}

impl From<parser::Dependency> for Dependency {
    fn from(dep: parser::Dependency) -> Self {
        Dependency {
            artifact_id: dep.artifact_id,
            dtype: dep.dtype,
            group_id: dep.group_id,
            optional: dep.optional,
            scope: dep.scope,
            version: dep.version,
        }
    }
}

#[derive(Debug)]
pub struct Build {
    pub default_goal: Option<String>,
    pub directory: Option<String>,
    pub extensions: Vec<Extension>,
    pub filters: Vec<String>,
    pub final_name: Option<String>,
    pub plugin_management: Vec<Plugin>,
    pub plugins: Vec<Plugin>,
    pub resources: Vec<Resource>,
    pub test_resources: Vec<Resource>,
}

impl From<parser::Build> for Build {
    fn from(build: parser::Build) -> Self {
        Build {
            default_goal: build.default_goal,
            directory: build.directory,
            extensions: match build.extensions {
                Some(es) => es.extensions.into_iter().map(|e| e.into()).collect(),
                None => Vec::new(),
            },
            filters: match build.filters {
                Some(fs) => fs.filters,
                None => Vec::new(),
            },
            final_name: build.final_name,
            plugin_management: match build.plugin_management {
                Some(ps) => ps.plugins.plugins.into_iter().map(|p| p.into()).collect(),
                None => Vec::new(),
            },
            plugins: match build.plugins {
                Some(ps) => ps.plugins.into_iter().map(|p| p.into()).collect(),
                None => Vec::new(),
            },
            resources: match build.resources {
                Some(rs) => rs.resources.into_iter().map(|r| r.into()).collect(),
                None => Vec::new(),
            },
            test_resources: match build.test_resources {
                Some(rs) => rs.test_resources.into_iter().map(|r| r.into()).collect(),
                None => Vec::new(),
            },
        }
    }
}

#[derive(Debug)]
pub struct Resource {
    pub directory: String,
    pub excludes: Vec<String>,
    pub filtering: Option<bool>,
    pub includes: Vec<String>,
    pub target_path: Option<String>,
}

impl From<parser::Resource> for Resource {
    fn from(resource: parser::Resource) -> Self {
        Resource {
            directory: resource.directory,
            excludes: match resource.excludes {
                Some(es) => es.excludes,
                None => Vec::new(),
            },
            filtering: resource.filtering,
            includes: match resource.includes {
                Some(is) => is.includes,
                None => Vec::new(),
            },
            target_path: resource.target_path,
        }
    }
}

#[derive(Debug)]
pub struct Plugin {
    pub artifact_id: String,
    pub configuration: Option<Configuration>,
    pub dependencies: Vec<Dependency>,
    pub executions: Vec<Execution>,
    pub extensions: Option<bool>,
    pub group_id: String,
    pub inherited: Option<bool>,
    pub version: Option<String>,
}

impl From<parser::Plugin> for Plugin {
    fn from(plugin: parser::Plugin) -> Self {
        Plugin {
            artifact_id: plugin.artifact_id,
            configuration: plugin.configuration.map(|c| c.into()),
            dependencies: match plugin.dependencies {
                Some(ds) => ds.dependencies.into_iter().map(|d| d.into()).collect(),
                None => Vec::new(),
            },
            executions: match plugin.executions {
                Some(es) => es.executions.into_iter().map(|e| e.into()).collect(),
                None => Vec::new(),
            },
            extensions: plugin.extensions,
            group_id: plugin.group_id,
            inherited: plugin.inherited,
            version: plugin.version,
        }
    }
}

#[derive(Debug)]
pub struct Execution {
    pub configuration: Option<Configuration>,
    pub goals: Vec<String>,
    pub id: Option<String>,
    pub inherited: Option<bool>,
    pub phase: Option<String>,
}

impl From<parser::Execution> for Execution {
    fn from(execution: parser::Execution) -> Self {
        Execution {
            configuration: execution.configuration.map(|c| c.into()),
            goals: match execution.goals {
                Some(gs) => gs.goals,
                None => Vec::new(),
            },
            id: execution.id,
            inherited: execution.inherited,
            phase: execution.phase,
        }
    }
}

#[derive(Debug)]
pub struct Configuration {
    // empty because this is different for every plugin and execution :(
}

impl From<parser::Configuration> for Configuration {
    fn from(_: parser::Configuration) -> Self {
        Configuration {}
    }
}

#[derive(Debug)]
pub struct Extension {
    pub artifact_id: String,
    pub group_id: String,
    pub version: Option<String>,
}

impl From<parser::Extension> for Extension {
    fn from(ext: parser::Extension) -> Self {
        Extension {
            artifact_id: ext.artifact_id,
            group_id: ext.group_id,
            version: ext.version,
        }
    }
}

#[derive(Debug)]
pub struct Profile {
    pub activation: Option<Activation>,
    pub id: Option<String>,
    pub dependency_management: Vec<Dependency>,
    pub modules: Vec<String>,
}

impl From<parser::Profile> for Profile {
    fn from(profile: parser::Profile) -> Self {
        Profile {
            activation: profile.activation.map(|a| a.into()),
            id: profile.id,
            dependency_management: match profile.dependency_management {
                Some(ds) => ds.dependencies.dependencies.into_iter().map(|d| d.into()).collect(),
                None => Vec::new(),
            },
            modules: match profile.modules {
                Some(ms) => ms.modules,
                None => Vec::new(),
            },
        }
    }
}

#[derive(Debug)]
pub struct Activation {
    pub active_by_default: Option<bool>,
}

impl From<parser::Activation> for Activation {
    fn from(activation: parser::Activation) -> Self {
        Activation {
            active_by_default: activation.active_by_default,
        }
    }
}

#[derive(Debug)]
pub struct Repository {
    pub id: String,
    pub url: String,
}

impl From<parser::Repository> for Repository {
    fn from(repo: parser::Repository) -> Self {
        Repository {
            id: repo.id,
            url: repo.url,
        }
    }
}
