use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotRef {
    pub id: String,
    pub vm_state_path: PathBuf,
    pub mem_path: PathBuf,
}

impl SnapshotRef {
    pub fn dir(&self) -> Option<&std::path::Path> {
        self.vm_state_path.parent()
    }
}
