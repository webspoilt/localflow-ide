use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use regex::Regex;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Permission {
    Allow,
    Deny,
    AuditRequired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRule {
    pub pattern: String,
    pub action: Permission,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicy {
    pub file_read_rules: Vec<SecurityRule>,
    pub file_write_rules: Vec<SecurityRule>,
    pub allowed_domains: Vec<String>,
    pub blocked_commands: Vec<String>,
    pub secret_patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityIncident {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub operation: String,
    pub detail: String,
    pub severity: String, // "Low" | "Medium" | "High" | "Critical"
    pub action_taken: String, // "Allowed" | "Blocked" | "Audited"
}

pub struct SecurityGraph {
    pub policy: SecurityPolicy,
    pub incidents: Vec<SecurityIncident>,
}

impl Default for SecurityGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl SecurityGraph {
    pub fn new() -> Self {
        let policy = SecurityPolicy {
            file_read_rules: vec![
                SecurityRule { pattern: r#"\.env$"#.to_string(), action: Permission::AuditRequired },
                SecurityRule { pattern: r#"\.ssh/"#.to_string(), action: Permission::Deny },
            ],
            file_write_rules: vec![
                SecurityRule { pattern: r#"\.env$"#.to_string(), action: Permission::AuditRequired },
                SecurityRule { pattern: r#"\.git/"#.to_string(), action: Permission::AuditRequired },
                SecurityRule { pattern: r#"\.ssh/"#.to_string(), action: Permission::Deny },
            ],
            allowed_domains: vec![
                "github.com".to_string(),
                "crates.io".to_string(),
                "npmjs.com".to_string(),
                "registry.npmjs.org".to_string(),
            ],
            blocked_commands: vec![
                "rm -rf /".to_string(),
                "format".to_string(),
                "mkfs".to_string(),
                "dd".to_string(),
            ],
            secret_patterns: vec![
                r#"(?i)api_key\s*=\s*['"][a-zA-Z0-9_\-]{16,}['"]"#.to_string(),
                r#"(?i)password\s*=\s*['"][a-zA-Z0-9_\-]{8,}['"]"#.to_string(),
                r#"(?i)sk-[a-zA-Z0-9]{48}"#.to_string(), // OpenAI secret key format
            ],
        };

        Self {
            policy,
            incidents: Vec::new(),
        }
    }

    /// Validates if a file write path complies with filesystem access policies
    pub fn validate_file_write(&mut self, path: &str) -> Permission {
        for rule in &self.policy.file_write_rules {
            if let Ok(re) = Regex::new(&rule.pattern) {
                if re.is_match(path) {
                    let incident = SecurityIncident {
                        id: uuid::Uuid::new_v4().to_string(),
                        timestamp: Utc::now(),
                        operation: "file_write".to_string(),
                        detail: format!("File write matched rule pattern: {}", rule.pattern),
                        severity: match rule.action {
                            Permission::Deny => "Critical".to_string(),
                            Permission::AuditRequired => "High".to_string(),
                            Permission::Allow => "Low".to_string(),
                        },
                        action_taken: match rule.action {
                            Permission::Deny => "Blocked".to_string(),
                            Permission::AuditRequired => "Audit Required".to_string(),
                            Permission::Allow => "Allowed".to_string(),
                        },
                    };
                    self.incidents.push(incident);
                    return rule.action;
                }
            }
        }
        Permission::Allow
    }

    /// Scans a content string for potential secret exposures (API keys, credentials, etc.)
    pub fn scan_content_for_secrets(&mut self, content: &str, location: &str) -> bool {
        let mut exposed = false;
        for pattern in &self.policy.secret_patterns {
            if let Ok(re) = Regex::new(pattern) {
                if re.is_match(content) {
                    exposed = true;
                    let incident = SecurityIncident {
                        id: uuid::Uuid::new_v4().to_string(),
                        timestamp: Utc::now(),
                        operation: "secret_leak_check".to_string(),
                        detail: format!("Potential secret signature matched in {}: {}", location, pattern),
                        severity: "High".to_string(),
                        action_taken: "Audit Flagged".to_string(),
                    };
                    self.incidents.push(incident);
                }
            }
        }
        exposed
    }

    /// Verifies if a shell command contains dangerous expressions
    pub fn validate_command(&mut self, command: &str) -> Result<(), String> {
        let cmd_lower = command.to_lowercase();
        for blocked in &self.policy.blocked_commands {
            if cmd_lower.contains(&blocked.to_lowercase()) {
                let incident = SecurityIncident {
                    id: uuid::Uuid::new_v4().to_string(),
                    timestamp: Utc::now(),
                    operation: "command_execution".to_string(),
                    detail: format!("Blocked destructive command attempt: {}", command),
                    severity: "Critical".to_string(),
                    action_taken: "Blocked".to_string(),
                };
                self.incidents.push(incident);
                return Err(format!("Command blocked by security sandbox: contains destructive sequence '{}'", blocked));
            }
        }
        Ok(())
    }

    /// Validates outbound networking host addresses
    pub fn validate_network_host(&mut self, host: &str) -> bool {
        let is_allowed = self.policy.allowed_domains.iter()
            .any(|d| host == d || host.ends_with(&format!(".{}", d)));

        if !is_allowed {
            let incident = SecurityIncident {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                operation: "network_outbound".to_string(),
                detail: format!("Unauthorized outbound connection blocked to target domain: {}", host),
                severity: "Medium".to_string(),
                action_taken: "Blocked".to_string(),
            };
            self.incidents.push(incident);
        }

        is_allowed
    }
}
