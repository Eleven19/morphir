use extism_pdk::*;
use morphir_codemodel::distribution::*;
use morphir_codemodel::project::MorphirProject;

#[plugin_fn]
pub fn greet(name: String) -> FnResult<String> {
    Ok(format!("Hello, {}!", name))
}

#[plugin_fn]
pub fn get_project_dependencies(
    Json(_project): Json<MorphirProject>,
) -> FnResult<Json<Vec<DistributionJson>>> {
    //let mut distributions = Vec::new();
    // for dependency in &project.dependencies {
    //     match &dependency.location {
    //         DependencyLocation::Local(path) => {
    //             let package_name: PackageName = path.into().clone();
    //             let info = ApplicationDistributionInfo { package_name };
    //             distributions.push(Distribution::Library(info));
    //         }
    //         DependencyLocation::Url(url) => {
    //             let package_name = PackageName(url.0.clone());
    //             let info = ApplicationDistributionInfo { package_name };
    //             distributions.push(Distribution::Library(info));
    //         }
    //     }
    // }
    let distributions: Vec<DistributionJson> = Vec::new();
    Ok(distributions.into())
}
