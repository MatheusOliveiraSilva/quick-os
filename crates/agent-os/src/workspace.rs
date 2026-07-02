use std::path::{Component, Path, PathBuf};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorkspaceError {
    #[error("path escapes workspace: {0}")]
    Escape(String),

    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

/// Sandboxed view of a single directory — agent cannot read outside it.
#[derive(Debug, Clone)]
pub struct Workspace {
    root: PathBuf,
}

impl Workspace {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn resolve(&self, relative: &str) -> Result<PathBuf, WorkspaceError> {
        let path = Path::new(relative);
        if path.is_absolute() {
            return Err(WorkspaceError::Escape(relative.to_string()));
        }

        for component in path.components() {
            if matches!(component, Component::ParentDir | Component::RootDir | Component::Prefix(_)) {
                return Err(WorkspaceError::Escape(relative.to_string()));
            }
        }

        Ok(self.root.join(path))
    }

    pub fn read_file(&self, relative: &str) -> Result<String, WorkspaceError> {
        let full = self.resolve(relative)?;
        std::fs::read_to_string(full).map_err(WorkspaceError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_parent_traversal() {
        let ws = Workspace::new("/tmp/ws");
        assert!(ws.resolve("../etc/passwd").is_err());
    }
}
