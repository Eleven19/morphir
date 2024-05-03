
use arcstr::ArcStr;
use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::path::{PathBuf};
use morphir_services::workspace::{WorkspaceDir, WorkspaceLocator, WorkspaceResolutionError};

pub const MORPHIR_HOST_ID_STR:ArcStr = arcstr::literal!("morphir");

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
/// A named identifier for a host.
///
/// # Examples
///
/// ```
/// # use morphir_hosting::host::HostId;
/// let morphir_host_id = HostId::MORPHIR;
/// let my_morphir_host_id = HostId::new("morphir");
/// assert_eq!(morphir_host_id, my_morphir_host_id);
/// ```
///
/// ```
/// # use morphir_hosting::host::HostId;
/// let host_id = HostId::new("my_host");
/// assert_eq!(host_id.id(), "my_host");
/// ```
///
pub struct HostId(ArcStr);
impl HostId {

    pub const MORPHIR:HostId = HostId(MORPHIR_HOST_ID_STR);
    pub fn new(id:&str) -> Self {
        Self(ArcStr::from(id))
    }

    pub fn id(&self) -> &str {
        &self.0
    }

}

pub struct Host {
    pub id:HostId,
}

impl Host {
    // pub fn locate_workspace_root<Locator>(path:OsString, locator:Locator) -> Result<WorkspaceDir, WorkspaceResolutionError> {
    //     todo!()
    // }

    pub fn new(id:HostId) -> Self {
        Self {
            id,
        }
    }

    /// Returns the name of the folder that should be used to store host-specific data.
    ///
    /// # Examples
    ///
    /// ```
    /// # use morphir_hosting::host::{HostId, Host};
    /// let morphir_host = Host::new(HostId::MORPHIR);
    /// assert_eq!(morphir_host.host_folder_name(), ".morphir");
    /// ```
    pub fn host_folder_name(&self) -> String {
        format!(".{}", self.id.id())
    }
}

impl WorkspaceLocator for Host {
    fn locate_workspace_root<P>(&self, path:P) -> Result<WorkspaceDir, WorkspaceResolutionError>
    where P:Into<PathBuf> {
        let host_folder_name = self.host_folder_name();
        let path = path.into();
        let mut current = path.to_owned();
        if current.is_relative() {
            let cwd = current_dir()?;
            current = cwd.join(current);
        }
        loop {
            if current.join(&host_folder_name).exists() {
                return Ok(WorkspaceDir::new(current));
            }
            if !current.pop() {
                break;
            }
        }
        let path_str = format!("{:?}",&path);
        Err(WorkspaceResolutionError{path:path_str})
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;
    use serde_json::*;
    use std::env::current_dir;

    #[test]
    fn a_host_id_should_serialize_to_a_simple_json_string(){
        let host_id = HostId::MORPHIR;
        let serialized = json!(host_id);
        let expected = json!("morphir");
        assert_eq!(serialized, expected);
    }

    #[test]
    fn it_should_be_possible_to_locate_the_workspace_root_for_a_host(){
        let host = Host::new(HostId::MORPHIR);
        let cwd = current_dir().unwrap();
        let path = cwd.join(file!());
        let result = host.locate_workspace_root(path);
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}