use crate::classic::module::ModuleName;
use crate::classic::name::Name;
use crate::classic::path::Path;
pub struct FQName((Path,Path,Name));
impl FQName {
    pub fn new(package_path: Path, module_path: Path, local_name: Name) -> Self {
        FQName((package_path, module_path, local_name))
    }

    pub fn local_name(&self) -> &Name {
        &self.0.2
    }

    // pub fn module_name(&self) -> ModuleName {
    //     let path = &self.0.1;
    //     ModuleName::from_path(&self.0.1)
    // }

    pub fn module_path(&self) -> &Path {
        &self.0.1
    }

    pub fn package_path(&self) -> &Path {
        &self.0.0
    }
}