use crate::workspace::WorkspaceDir;

pub struct NoWorkspaceDir;

#[derive(Debug, Clone)]
pub struct WorkspaceBuilder<WsDir> {
    root: WsDir,
}

impl WorkspaceBuilder<NoWorkspaceDir> {
    pub fn new() -> Self {
        Self {
            root: NoWorkspaceDir,
        }
    }

    pub fn root<P: Into<WorkspaceDir>>(self, root: P) -> WorkspaceBuilder<WorkspaceDir> {
        WorkspaceBuilder {
            root: root.into(),
        }
    }
}