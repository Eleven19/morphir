use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;
use thiserror::Error;

pub struct WorkspaceDir(Arc<Path>);

impl WorkspaceDir {
    pub fn new(path:&Path) -> Self {
        Self(path.into())
    }

    pub fn from_path(path:&Path) -> Option<Self> {
        if path.is_dir() {
            Some(Self::new(path))
        } else {
            let dir = path.parent()?;
            Some(Self::new(dir))
        }
    }

    pub fn as_os_str(&self) -> &OsStr {
        self.0.as_os_str()
    }

    pub fn as_path(&self) -> &Path {
        &self.0
    }
}

impl FromStr for WorkspaceDir {
    type Err = WorkspaceResolutionError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let path = Path::new(input);
        if path.is_dir() {
            Ok(Self::new(path))
        } else if let Some(dir) = path.parent() {
            Ok(Self::new(dir.into()))
        } else {
            let str_path = path.display().to_string();
            Err(WorkspaceResolutionError::WorkspaceUnresolvable(str_path))
        }
    }
}

pub trait WorkspaceLocator{
    fn locate_workspace_root<P>(&self, path:P) -> Result<WorkspaceDir, WorkspaceResolutionError>
        where P:Into<PathBuf>;
}

#[derive(Debug, Error)]
pub enum WorkspaceResolutionError {
    #[error("unable to resolve workspace directory given path {0}")]
    WorkspaceUnresolvable(String),
    #[error("unable to find workspace directory at path {0}")]
    WorkspaceDirNotFound(String),
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::str::FromStr;
    use crate::workspace::WorkspaceDir;

    #[test]
    fn workspace_dir_should_be_friendly_to_ownership_changes() {
        let workspace_dir = WorkspaceDir::from_str(file!()).unwrap();
        let expected = Path::new(file!()).parent().unwrap();
        assert_eq!(workspace_dir.as_path(), expected);
    }
}
