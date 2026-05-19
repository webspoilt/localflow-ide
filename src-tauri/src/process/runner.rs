use tokio::process::Command;
use tracing::{info, debug};
use anyhow::Result;

use crate::core::TaskDefinition;

#[derive(Debug, Clone)]
pub struct ProcessOutput {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u64,
}

#[derive(Clone)]
pub struct ProcessRunner;

impl ProcessRunner {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute(&self, definition: &TaskDefinition) -> Result<ProcessOutput> {
        let start = std::time::Instant::now();

        let command_str = definition
            .command
            .as_deref()
            .unwrap_or_default();

        if command_str.is_empty() {
            anyhow::bail!("No command specified");
        }

        if cfg!(target_os = "windows") {
            let mut cmd = Command::new("cmd");
            cmd.arg("/C").arg(command_str);
            cmd.args(&definition.args);
            if let Some(cwd) = &definition.cwd {
                cmd.current_dir(cwd);
            }
            for (key, value) in &definition.env {
                cmd.env(key, value);
            }

            debug!(command = %command_str, "Spawning process");
            let output = cmd.output().await
                .map_err(|e| anyhow::anyhow!("Failed to execute command '{}': {}", command_str, e))?;

            let duration = start.elapsed().as_millis() as u64;
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let exit_code = output.status.code().unwrap_or(-1);

            info!(command = %command_str, exit_code, duration_ms = duration, "Process completed");

            Ok(ProcessOutput { exit_code, stdout, stderr, duration_ms: duration })
        } else {
            let mut cmd = Command::new("sh");
            cmd.arg("-c").arg(command_str);
            cmd.args(&definition.args);
            if let Some(cwd) = &definition.cwd {
                cmd.current_dir(cwd);
            }
            for (key, value) in &definition.env {
                cmd.env(key, value);
            }

            debug!(command = %command_str, "Spawning process");
            let output = cmd.output().await
                .map_err(|e| anyhow::anyhow!("Failed to execute command '{}': {}", command_str, e))?;

            let duration = start.elapsed().as_millis() as u64;
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let exit_code = output.status.code().unwrap_or(-1);

            info!(command = %command_str, exit_code, duration_ms = duration, "Process completed");

            Ok(ProcessOutput { exit_code, stdout, stderr, duration_ms: duration })
        }
    }
}
