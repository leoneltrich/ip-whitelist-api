use super::FirewallBackend;
use async_trait::async_trait;
use shared::errors::app_errors::AppError;
use std::net::IpAddr;
use std::process::Command;
use std::time::Duration;
use tracing::log::debug;
use tracing::{error, info};

pub struct NftablesFirewall {
    table: String,
    chain: String,
    set: String,
    api_port: u16,
}

impl NftablesFirewall {
    pub fn new() -> Self {
        Self {
            table: "filter".to_string(),
            chain: "input".to_string(),
            set: "allowed_clients".to_string(),
            api_port: 3000,
        }
    }

    /// Helper to execute an nft command safely
    fn run_nft_cmd(&self, args: &[&str]) -> Result<(), AppError> {
        debug!("Running nft command with args: {:?}", args);
        let output = Command::new("nft").args(args).output().map_err(|e| {
            error!("Failed to run nft command: {}", e);
            AppError::InternalServerError("An internal server error occurred".to_string())
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("nft command failed with error: {}", stderr);
            return Err(AppError::InternalServerError(
                "An internal server error occurred".to_string(),
            ));
        }

        Ok(())
    }
}

#[async_trait]
impl FirewallBackend for NftablesFirewall {
    async fn setup(&self) -> Result<(), AppError> {
        info!("Integrating with Main NFTables (inet filter)...");

        self.run_nft_cmd(&[
            "add",
            "set",
            "inet",
            &self.table,
            &self.set,
            "{ type ipv4_addr . inet_service; flags timeout; }",
        ])?;

        let set_ref = format!("@{}", self.set);

        let check_cmd = Command::new("nft")
            .args(&["list", "chain", "inet", &self.table, &self.chain])
            .output()
            .map_err(|e| {
                error!("Failed to list chain: {}", e);
                AppError::InternalServerError("An internal server error occurred".to_string())
            })?;

        let existing_rules = String::from_utf8_lossy(&check_cmd.stdout);

        if !existing_rules.contains(&set_ref) {
            info!("Rule not found. Inserting whitelist rule...");
            self.run_nft_cmd(&[
                "insert",
                "rule",
                "inet",
                &self.table,
                &self.chain,
                "ip",
                "saddr",
                ".",
                "tcp",
                "dport",
                &set_ref,
                "accept",
            ])?;
        } else {
            info!("Whitelist rule already exists in the main chain. Skipping insertion.");
        }

        Ok(())
    }

    async fn grant_access(
        &self,
        ip: IpAddr,
        port: u16,
        duration: Duration,
    ) -> Result<(), AppError> {
        let ip_str = ip.to_string();
        let port_str = port.to_string();
        let timeout_str = format!("{}s", duration.as_secs());

        // Element format: { 1.2.3.4 . 8080 }
        let element_content = format!("{} . {}", ip_str, port_str);

        // (Reset Timer)
        let _ = Command::new("nft")
            .args(&[
                "delete",
                "element",
                "inet",
                &self.table,
                &self.set,
                &format!("{{ {} }}", element_content),
            ])
            .output();

        let element_with_timeout = format!("{{ {} timeout {} }}", element_content, timeout_str);

        info!(
            "Granting access to IP: {} on PORT: {} for {:?}",
            ip, port, duration
        );
        self.run_nft_cmd(&[
            "add",
            "element",
            "inet",
            &self.table,
            &self.set,
            &element_with_timeout,
        ])
    }

    async fn validate_config(&self) -> Result<(), AppError> {
        let version_check = Command::new("nft").arg("--version").output();
        if version_check.is_err() {
            error!("nft binary not found");
            return Err(AppError::InternalServerError(
                "An internal server error occurred".to_string(),
            ));
        }
        Ok(())
    }
}
