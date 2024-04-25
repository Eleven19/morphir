use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageName(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleName(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct DependencyPath(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct DependencyInfo {
    location: DependencyLocation,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DependencyLocation {
    Local(DependencyPath),
    Url(Url),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MorphirProject {
    name: PackageName,
    #[serde(rename = "sourceDirectory")]
    source_directory: String,
    #[serde(rename = "exposedModules")]
    exposed_modules: Vec<ModuleName>,
    #[serde(rename = "localDependencies", default)]
    local_dependencies: Vec<DependencyPath>,
    #[serde(default)]
    dependencies: Vec<DependencyInfo>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_name_serialization() {
        let package_name = PackageName("test".to_string());
        let serialized = serde_json::to_string(&package_name).unwrap();
        assert_eq!(serialized, "\"test\"");
    }

    #[test]
    fn test_package_name_deserialization() {
        let serialized = "\"test\"";
        let package_name: PackageName = serde_json::from_str(serialized).unwrap();
        assert_eq!(package_name.0, "test");
    }

    #[test]
    fn can_deserialize_a_morphir_project_from_json_when_local_dependencies_is_not_in_json() {
        let json = r#"{
            "name": "test",
            "sourceDirectory": "src",
            "exposedModules": []
        }"#;
        let project: MorphirProject = serde_json::from_str(json).unwrap();
        assert_eq!(project.name.0, "test");
        assert_eq!(project.source_directory, "src");
        assert_eq!(project.exposed_modules.len(), 0);
        assert_eq!(project.local_dependencies.len(), 0);
    }
}
