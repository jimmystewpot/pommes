//#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(clippy::result_unwrap_used)]
#![warn(clippy::option_unwrap_used)]

use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Project {
    #[serde(rename = "modelVersion")]
    pub model_version: String,
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    pub version: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "inceptionYear")]
    pub inception_year: Option<String>,
    pub licenses: Option<Licenses>,
    pub organization: Option<Organization>,
    pub developers: Option<Developers>,
    pub contributors: Option<Contributors>,
    pub packaging: Option<String>,
    pub parent: Option<Parent>,
    pub dependencies: Option<Dependencies>,
    #[serde(rename = "dependency_management")]
    pub dependency_management: Option<Dependencies>,
    pub modules: Option<Modules>,
    pub build: Option<Build>,
    pub profiles: Option<Profiles>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Licenses {
    #[serde(rename = "license")]
    pub licenses: Vec<License>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct License {
    pub name: String,
    pub url: String,
    pub distribution: String,
    pub comments: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Organization {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Developers {
    #[serde(rename = "developer")]
    pub developers: Vec<Person>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Contributors {
    #[serde(rename = "contributor")]
    pub contributors: Vec<Person>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Person {
    pub id: Option<String>,
    pub name: String,
    pub email: Option<String>,
    pub url: Option<String>,
    pub organization: Option<String>,
    #[serde(rename = "organizationUrl")]
    pub organization_url: Option<String>,
    pub roles: Option<Roles>,
    pub timezone: Option<String>,
    #[serde(default)]
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Roles {
    #[serde(rename = "role")]
    pub roles: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Parent {
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    pub version: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Dependencies {
    #[serde(rename = "dependency", default)]
    pub dependencies: Vec<Dependency>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Dependency {
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    #[serde(rename = "version")]
    pub version: Option<String>,
    #[serde(rename = "type")]
    pub dtype: Option<String>,
    #[serde(rename = "scope")]
    pub scope: Option<String>,
    #[serde(rename = "optional")]
    pub optional: Option<bool>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Modules {
    #[serde(rename = "module", default)]
    pub modules: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Build {
    #[serde(rename = "defaultGoal")]
    pub default_goal: Option<String>,
    pub directory: Option<String>,
    #[serde(rename = "finalName")]
    pub final_name: Option<String>,
    pub filters: Option<Filters>,
    pub resources: Option<Resources>,
    #[serde(rename = "testResources")]
    pub test_resources: Option<TestResources>,
    pub plugins: Option<Plugins>,
    #[serde(rename = "pluginManagement")]
    pub plugin_management: Option<PluginManagement>,
    pub extensions: Option<Extensions>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Filters {
    #[serde(rename = "filter")]
    pub filters: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Resources {
    #[serde(rename = "resource")]
    pub resources: Vec<Resource>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TestResources {
    #[serde(rename = "testResource")]
    pub test_resources: Vec<Resource>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Resource {
    #[serde(rename = "targetPath")]
    pub target_path: Option<String>,
    pub filtering: Option<bool>,
    pub directory: String,
    pub includes: Option<Includes>,
    pub excludes: Option<Excludes>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Includes {
    #[serde(rename = "include")]
    pub includes: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Excludes {
    #[serde(rename = "exclude")]
    pub excludes: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Plugins {
    #[serde(rename = "plugin")]
    pub plugins: Vec<Plugin>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct PluginManagement {
    pub plugins: Plugins,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Plugin {
    #[serde(rename = "groupId", default = "default_plugin_group_id")]
    pub group_id: String,
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    pub version: Option<String>,
    pub extension: Option<bool>,
    pub inherited: Option<bool>,
    pub dependencies: Option<Dependencies>,
    pub executions: Option<Executions>,
}

fn default_plugin_group_id() -> String {
    String::from("org.apache.maven.plugins")
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Executions {
    #[serde(rename = "execution")]
    pub executions: Vec<Execution>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Execution {
    pub id: String,
    pub goals: Goals,
    pub phase: String,
    pub inherited: Option<bool>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Goals {
    #[serde(rename = "goal")]
    pub goals: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Extensions {
    #[serde(rename = "extension")]
    pub extensions: Vec<Extension>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Extension {
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    pub version: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Profiles {
    #[serde(rename = "profile")]
    pub profiles: Vec<Profile>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Profile {
    pub id: String,
    pub activation: Option<Activation>,
    pub modules: Option<Modules>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Activation {
    #[serde(rename = "activeByDefault")]
    pub active_by_default: Option<bool>,
}

#[cfg(test)]
mod tests;
