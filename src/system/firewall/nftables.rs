use async_trait::async_trait;
use std::net::IpAddr;
use std::process::Command;
use std::time::Duration;
use crate::errors::AppError;
use super::FirewallBackend;

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
        let output = Command::new("nft")
            .args(args)
            .output()
            .map_err(|e| AppError::InternalServerError(format!("Failed to execute nft: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::InternalServerError(format!("NFTables error: {}", stderr)));
        }

        Ok(())
    }
}

#[async_trait]
impl FirewallBackend for NftablesFirewall {
    async fn setup(&self) -> Result<(), AppError> {
        println!("Integrating with Main NFTables (inet filter)...");


        self.run_nft_cmd(&[
            "add", "set", "inet", &self.table, &self.set,
            "{ type ipv4_addr . inet_service; flags timeout; }"
        ])?;

        let set_ref = format!("@{}", self.set);

        let check_cmd = Command::new("nft")
            .args(&["list", "chain", "inet", &self.table, &self.chain])
            .output()
            .map_err(|e| AppError::InternalServerError(format!("Failed to check nft: {}", e)))?;

        let existing_rules = String::from_utf8_lossy(&check_cmd.stdout);

        if !existing_rules.contains(&set_ref) {
            println!("Rule not found. Inserting whitelist rule...");
            self.run_nft_cmd(&[
                "insert", "rule", "inet", &self.table, &self.chain,
                "ip", "saddr", ".", "tcp", "dport", &set_ref, "accept"
            ])?;
        } else {
            println!("Whitelist rule already exists in the main chain. Skipping insertion.");
        }

        Ok(())
    }

    async fn grant_access(&self, ip: IpAddr, port: u16, duration: Duration) -> Result<(), AppError> {
        let ip_str = ip.to_string();
        let port_str = port.to_string();
        let timeout_str = format!("{}s", duration.as_secs());

        // Element format: { 1.2.3.4 . 8080 }
        let element_content = format!("{} . {}", ip_str, port_str);

        // 1. DELETE (Reset Timer)
        let _ = Command::new("nft")
            .args(&[
                "delete", "element", "inet", &self.table, &self.set,
                &format!("{{ {} }}", element_content)
            ])
            .output();

        let element_with_timeout = format!("{{ {} timeout {} }}", element_content, timeout_str);

        self.run_nft_cmd(&[
            "add", "element", "inet", &self.table, &self.set, &element_with_timeout
        ])
    }

    // (validate_config remains the same...)
    async fn validate_config(&self) -> Result<(), AppError> {
        let version_check = Command::new("nft").arg("--version").output();
        if version_check.is_err() {
            return Err(AppError::InternalServerError("nft binary not found".to_string()));
        }
        Ok(())
    }
}