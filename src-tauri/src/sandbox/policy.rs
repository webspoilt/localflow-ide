use std::collections::HashSet;
use tracing::{info, warn};

/// Enforces sandbox policies for command execution.
pub struct Sandbox {
    allowed_commands: HashSet<String>,
    denied_commands: HashSet<String>,
    allowed_paths: Vec<String>,
    enable_network: bool,
    enable_filesystem: bool,
}

impl Sandbox {
    pub fn new() -> Self {
        let mut denied = HashSet::new();
        denied.insert("rm".to_string());
        denied.insert("sudo".to_string());
        denied.insert("su".to_string());
        denied.insert("chmod".to_string());
        denied.insert("chown".to_string());
        denied.insert("mkfs".to_string());
        denied.insert("dd".to_string());
        denied.insert("shutdown".to_string());
        denied.insert("reboot".to_string());
        denied.insert("poweroff".to_string());
        denied.insert("init".to_string());
        denied.insert("sysctl".to_string());
        denied.insert("passwd".to_string());
        denied.insert("useradd".to_string());
        denied.insert("usermod".to_string());
        denied.insert("groupadd".to_string());

        Self {
            allowed_commands: HashSet::new(),
            denied_commands: denied,
            allowed_paths: vec![],
            enable_network: false,
            enable_filesystem: true,
        }
    }

    pub fn with_allowed_commands(mut self, commands: Vec<String>) -> Self {
        self.allowed_commands = commands.into_iter().collect();
        self
    }

    pub fn with_network(mut self, enable: bool) -> Self {
        self.enable_network = enable;
        self
    }

    /// Validate a command string against sandbox policy.
    /// Returns Ok(()) if allowed, or an error string explaining why denied.
    pub fn validate_command(&self, command: &str) -> Result<(), String> {
        let trimmed = command.trim();

        if trimmed.is_empty() {
            return Err("Empty command".to_string());
        }

        // Extract the base command (first word)
        let base_cmd = trimmed
            .split_whitespace()
            .next()
            .unwrap_or("");

        // Check denied commands first
        if self.denied_commands.contains(base_cmd) {
            warn!(command = %base_cmd, "Blocked by deny list");
            return Err(format!("Command '{}' is not allowed for security reasons", base_cmd));
        }

        // If allowed_commands is non-empty, enforce allowlist
        if !self.allowed_commands.is_empty() && !self.allowed_commands.contains(base_cmd) {
            warn!(command = %base_cmd, "Not in allow list");
            return Err(format!(
                "Command '{}' is not in the allowed commands list",
                base_cmd
            ));
        }

        // Check for dangerous patterns
        let dangerous_patterns = [
            "rm -rf /",
            "rm -rf /*",
            "> /dev/sda",
            "| sudo",
            "| su ",
            "; sudo",
            "; su ",
            "&& sudo",
            "&& su ",
            "`sudo",
            "$(sudo",
            "wget ",
            "curl ",
            "nc ",
        ];

        for pattern in &dangerous_patterns {
            if trimmed.contains(pattern) {
                warn!(pattern = %pattern, "Dangerous pattern detected");
                return Err(format!("Command contains dangerous pattern: '{}'", pattern));
            }
        }

        info!(command = %command, "Sandbox validation passed");
        Ok(())
    }

    /// Validate a file path against allowed paths
    pub fn validate_path(&self, path: &str) -> Result<(), String> {
        if !self.enable_filesystem {
            return Err("Filesystem access is disabled".to_string());
        }

        if self.allowed_paths.is_empty() {
            return Ok(()); // No restrictions on paths
        }

        let path_std = path.replace('\\', "/");
        for allowed in &self.allowed_paths {
            if path_std.starts_with(allowed) {
                return Ok(());
            }
        }

        Err(format!("Path '{}' is not in allowed paths", path))
    }
}
