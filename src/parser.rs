use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Project {
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    pub build: Option<Build>,
    pub contributors: Option<Contributors>,
    pub dependencies: Option<Dependencies>,
    #[serde(rename = "dependencyManagement")]
    pub dependency_management: Option<Dependencies>,
    pub description: Option<String>,
    pub developers: Option<Developers>,
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    #[serde(rename = "inceptionYear")]
    pub inception_year: Option<String>,
    #[serde(rename = "issueManagement")]
    pub issue_management: Option<IssueManagement>,
    pub licenses: Option<Licenses>,
    #[serde(rename = "modelVersion")]
    pub model_version: String,
    pub modules: Option<Modules>,
    pub name: Option<String>,
    pub organization: Option<Organization>,
    pub packaging: Option<String>,
    pub parent: Option<Parent>,
    #[serde(rename = "pluginRepositories")]
    pub plugin_repositories: Option<PluginRepositories>,
    pub profiles: Option<Profiles>,
    #[serde(default)]
    pub properties: HashMap<String, String>,
    pub repositories: Option<Repositories>,
    pub url: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Licenses {
    #[serde(rename = "license", default)]
    pub licenses: Vec<License>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct License {
    pub comments: Option<String>,
    pub distribution: Option<String>,
    pub name: String,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct IssueManagement {
    pub system: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Organization {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Developers {
    #[serde(rename = "developer", default)]
    pub developers: Vec<Person>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Contributors {
    #[serde(rename = "contributor", default)]
    pub contributors: Vec<Person>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Person {
    pub email: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub organization: Option<String>,
    #[serde(rename = "organizationUrl")]
    pub organization_url: Option<String>,
    #[serde(default)]
    pub properties: HashMap<String, String>,
    pub roles: Option<Roles>,
    pub timezone: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Roles {
    #[serde(rename = "role", default)]
    pub roles: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Parent {
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    #[serde(rename = "groupId")]
    pub group_id: String,
    pub version: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Dependencies {
    #[serde(rename = "dependency", default)]
    pub dependencies: Vec<Dependency>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Dependency {
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    #[serde(rename = "type")]
    pub dtype: Option<String>,
    #[serde(rename = "groupId")]
    pub group_id: String,
    pub optional: Option<bool>,
    pub scope: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Modules {
    #[serde(rename = "module", default)]
    pub modules: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Build {
    #[serde(rename = "defaultGoal")]
    pub default_goal: Option<String>,
    pub directory: Option<String>,
    pub extensions: Option<Extensions>,
    pub filters: Option<Filters>,
    #[serde(rename = "finalName")]
    pub final_name: Option<String>,
    #[serde(rename = "pluginManagement")]
    pub plugin_management: Option<PluginManagement>,
    pub plugins: Option<Plugins>,
    pub resources: Option<Resources>,
    #[serde(rename = "testResources")]
    pub test_resources: Option<TestResources>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Filters {
    #[serde(rename = "filter", default)]
    pub filters: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Resources {
    #[serde(rename = "resource", default)]
    pub resources: Vec<Resource>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct TestResources {
    #[serde(rename = "testResource", default)]
    pub test_resources: Vec<Resource>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Resource {
    pub directory: String,
    pub excludes: Option<Excludes>,
    pub filtering: Option<bool>,
    pub includes: Option<Includes>,
    #[serde(rename = "targetPath")]
    pub target_path: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Includes {
    #[serde(rename = "include", default)]
    pub includes: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Excludes {
    #[serde(rename = "exclude", default)]
    pub excludes: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Plugins {
    #[serde(rename = "plugin", default)]
    pub plugins: Vec<Plugin>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct PluginManagement {
    pub plugins: Plugins,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Plugin {
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    pub configuration: Option<Configuration>,
    pub dependencies: Option<Dependencies>,
    pub executions: Option<Executions>,
    pub extensions: Option<bool>,
    #[serde(rename = "groupId", default = "default_plugin_group_id")]
    pub group_id: String,
    pub inherited: Option<bool>,
    pub version: Option<String>,
}

fn default_plugin_group_id() -> String {
    String::from("org.apache.maven.plugins")
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Executions {
    #[serde(rename = "execution", default)]
    pub executions: Vec<Execution>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Execution {
    pub configuration: Option<Configuration>,
    pub goals: Option<Goals>,
    pub id: Option<String>,
    pub inherited: Option<bool>,
    pub phase: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Configuration {
    // empty because this is different for every plugin and execution :(
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Goals {
    #[serde(rename = "goal", default)]
    pub goals: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Extensions {
    #[serde(rename = "extension", default)]
    pub extensions: Vec<Extension>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Extension {
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    #[serde(rename = "groupId")]
    pub group_id: String,
    pub version: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Profiles {
    #[serde(rename = "profile", default)]
    pub profiles: Vec<Profile>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Profile {
    pub activation: Option<Activation>,
    pub id: Option<String>,
    #[serde(rename = "dependencyManagement")]
    pub dependency_management: Option<DependencyManagement>,
    pub modules: Option<Modules>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Activation {
    #[serde(rename = "activeByDefault")]
    pub active_by_default: Option<bool>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct DependencyManagement {
    pub dependencies: Dependencies,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Repositories {
    #[serde(rename = "repository", default)]
    pub repositories: Vec<Repository>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct PluginRepositories {
    #[serde(rename = "pluginRepository", default)]
    pub repositories: Vec<Repository>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Repository {
    pub id: String,
    pub url: String,
}
