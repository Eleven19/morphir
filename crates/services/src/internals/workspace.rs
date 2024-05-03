use serde::{Deserialize, Serialize};
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorkspaceResolutionError {
    #[error("unable to resolve workspace directory from path {0}")]
    WorkspaceUnresolvable(String),
}

#[derive(Clone,Debug,PartialEq,Eq,Hash,Ord,PartialOrd,Serialize,Deserialize)]
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
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = Path::new(s);
        if path.is_dir() {
            Ok(Self::new(path))
        } else if let Some(dir) = path.parent() {
            Ok(Self::new(dir.into()))
        } else {
            let str_path = s.to_string();
            Err(WorkspaceResolutionError::WorkspaceUnresolvable(str_path))
        }
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