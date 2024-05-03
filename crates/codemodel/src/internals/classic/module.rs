use crate::classic::path::Path;
pub struct ModuleName(Path);
impl ModuleName {
    pub fn new(path: Path) -> Self {
        ModuleName(path)
    }

    pub fn from_path(path: Path) -> Self {
        ModuleName(path)
    }

    pub fn path(&self) -> &Path {
        &self.0
    }
}

pub struct QualifiedModuleName((Path, Path));
