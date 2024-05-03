use std::ffi::OsString;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorkspaceResolutionError {
    #[error("unable to resolve workspace directory from path {0}")]
    WorkspaceNotFound(),
}

#[derive(Debug)]
pub struct WorkspaceDir(PathBuf);

impl WorkspaceDir {
    pub fn new(path:PathBuf) -> Self {
        Self(path)
    }

    pub fn from_path(path:&Path) -> Option<Self> {
        if path.is_dir() {
            Some(Self::new(path.to_owned()))
        } else {
            let dir = path.parent()?;
            Some(Self::new(dir.to_owned()))
        }
    }

    pub fn into_os_string(self) -> OsString {
        self.0.into_os_string()
    }

    pub fn as_path(&self) -> &Path {
        &self.0
    }
}

impl FromStr for WorkspaceDir {
    type Err = WorkspaceResolutionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path_buf = PathBuf::from(s);
        if path_buf.is_dir() {
            Ok(Self::new(path_buf))
        } else if let Some(dir) = path_buf.parent() {
            Ok(Self::new(dir.to_owned()))
        } else {
            Err(WorkspaceResolutionError{path:s.to_owned()})
        }
    }
}

impl From<WorkspaceDir> for OsString {
    fn from(dir:WorkspaceDir) -> Self {
        dir.into_os_string()
    }
}

#[derive(Debug)]
pub struct Workspace {
    workspace_root: Arc<PathBuf>,
}

impl Workspace {
    pub fn new(root:OsString) -> Self {
        let path = Path::new(&root);
        let workspace_root = Arc::new(path.to_owned());
        
        Self {
            workspace_root,
        }
    }

    pub fn find_containing_workspace_of(path:&Path) -> Option<Workspace> {
        let mut current = path.to_owned();
        loop {
            if current.join("morphir.toml").exists() {
                return Some(Workspace::new(current.into_os_string()));
            }
            if !current.pop() {
                break;
            }
        }
        None
    }
}

pub trait WorkspaceLocator{
    fn locate_workspace_root<P>(&self, path:P) -> Result<WorkspaceDir, WorkspaceResolutionError>
    where P:Into<PathBuf>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;

    #[test]
    fn test_workspace_new() {
        let root = OsString::from("test");
        let workspace = Workspace::new(root);
        assert_eq!(workspace.workspace_root.to_str().unwrap(), "test");
    }

    #[test]
    fn workspace_dir_should_be_friendly_to_ownership_changes() {
        let workspace_dir = WorkspaceDir::from_str(file!()).unwrap();
        let expected = Path::new(file!()).parent().unwrap();
        assert_eq!(workspace_dir.as_path(), expected);
    }
}