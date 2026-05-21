use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualFileSystem {
    pub mounts: HashMap<String, String>, // virtual_path -> file content
    pub base_physical_path: PathBuf,
}

pub struct SandboxV2 {
    pub virtual_fs: VirtualFileSystem,
    pub network_isolated: bool,
    pub resource_ram_limit_mb: u64,
}

impl Default for SandboxV2 {
    fn default() -> Self {
        Self::new()
    }
}

impl SandboxV2 {
    pub fn new() -> Self {
        Self {
            virtual_fs: VirtualFileSystem {
                mounts: HashMap::new(),
                base_physical_path: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            },
            network_isolated: true,
            resource_ram_limit_mb: 2048,
        }
    }

    /// Set virtual file content without touching physical disk
    pub fn virtual_write(&mut self, virtual_path: &str, content: &str) {
        self.virtual_fs.mounts.insert(virtual_path.to_string(), content.to_string());
    }

    /// Read file content, prioritizing the virtual filesystem layer, falling back to physical disk
    pub fn virtual_read(&self, file_path: &str) -> Result<String, String> {
        if let Some(content) = self.virtual_fs.mounts.get(file_path) {
            return Ok(content.clone());
        }

        // Fallback to physical read
        let physical = self.virtual_fs.base_physical_path.join(file_path);
        if physical.exists() && physical.is_file() {
            std::fs::read_to_string(physical).map_err(|e| e.to_string())
        } else {
            Err(format!("File not found: {}", file_path))
        }
    }

    /// Commits virtual changes to the physical disk (usually gated by approval)
    pub fn commit_virtual_changes(&mut self) -> Result<(), String> {
        for (vpath, content) in &self.virtual_fs.mounts {
            let ppath = self.virtual_fs.base_physical_path.join(vpath);
            if let Some(parent) = ppath.parent() {
                std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            std::fs::write(ppath, content).map_err(|e| e.to_string())?;
        }
        self.virtual_fs.mounts.clear();
        Ok(())
    }

    /// Toggles networking status
    pub fn configure_networking(&mut self, isolated: bool) {
        self.network_isolated = isolated;
    }

    /// Verifies if a shell command requires explicit human authorization
    pub fn is_destructive_action(&self, command: &str) -> bool {
        let cmd = command.to_lowercase();
        cmd.contains("rm ") || cmd.contains("del ") || cmd.contains("format") || cmd.contains("git reset --hard") || cmd.contains("git clean")
    }
}
