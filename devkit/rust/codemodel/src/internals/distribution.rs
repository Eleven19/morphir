use crate::PackageName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Distribution {
    Application(ApplicationDistributionInfo),
    Bundle(BundleDistributionInfo),
    Library(ApplicationDistributionInfo),
}

impl Distribution {
    pub fn package_name(&self) -> &PackageName {
        match self {
            Distribution::Application(info) => &info.package_name,
            Distribution::Bundle(info) => &info.package_name,
            Distribution::Library(info) => &info.package_name,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Distributions(pub Vec<Distribution>);
impl Distributions {
    pub fn empty() -> Self {
        Distributions(Vec::new())
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ApplicationDistributionInfo {
    pub package_name: PackageName,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct BundleDistributionInfo {
    pub package_name: PackageName,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct LibraryDistributionInfo {
    pub package_name: PackageName,
}

#[cfg(feature = "json")]
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct DistributionJson(serde_json::Value);
