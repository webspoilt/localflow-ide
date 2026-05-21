use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Capability {
    FilesystemRead,
    FilesystemWrite,
    Network,
    Terminal,
    Process,
    ModelAccess,
    Plugin,
    Admin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilitySet {
    pub capabilities: Vec<Capability>,
    pub requires_approval: Vec<Capability>,
}

impl CapabilitySet {
    pub fn default_agent() -> Self {
        Self {
            capabilities: vec![
                Capability::FilesystemRead,
                Capability::FilesystemWrite,
                Capability::Terminal,
                Capability::ModelAccess,
            ],
            requires_approval: vec![Capability::FilesystemWrite],
        }
    }

    pub fn has(&self, cap: &Capability) -> bool {
        self.capabilities.contains(cap)
    }

    pub fn needs_approval(&self, cap: &Capability) -> bool {
        self.requires_approval.contains(cap)
    }
}
