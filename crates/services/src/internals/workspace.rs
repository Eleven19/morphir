use serde::{Deserialize, Serialize};
use std::ffi::{OsString};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use morphir_codemodel::workspace::*;

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


#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;
    use std::str::FromStr;

    #[test]
    fn test_workspace_new() {
        let root = OsString::from("test");
        let workspace = Workspace::new(root);
        assert_eq!(workspace.workspace_root.to_str().unwrap(), "test");
    }

}