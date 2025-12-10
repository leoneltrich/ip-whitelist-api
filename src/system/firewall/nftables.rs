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
            table: "access_proxy".to_string(),
            chain: "filter_input".to_string(),
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
    async fn grant_access(&self, ip: IpAddr, duration: Duration) -> Result<(), AppError> {
        let ip_str = ip.to_string();
        let timeout_str = format!("{}s", duration.as_secs());

        let delete_args = [
            "delete", "element",
            "inet", &self.table,
            &self.set,
            &format!("{{ {} }}", ip_str)
        ];

        let _ = Command::new("nft")
            .args(&delete_args)
            .output();

        let element = format!("{{ {} timeout {} }}", ip_str, timeout_str);
        let add_args = [
            "add", "element",
            "inet", &self.table,
            &self.set,
            &element
        ];

        self.run_nft_cmd(&add_args)
    }

    async fn validate_config(&self) -> Result<(), AppError> {
        // Check 1: Is 'nft' installed?
        let version_check = Command::new("nft").arg("--version").output();
        if version_check.is_err() {
            return Err(AppError::InternalServerError("nft binary not found".to_string()));
        }

        // Check 2: Do the table and set exist?
        // Command: nft list set inet <table_name> <set_name>
        let args = ["list", "set", "inet", &self.table, &self.set];

        match self.run_nft_cmd(&args) {
            Ok(_) => Ok(()),
            Err(_) => Err(AppError::InternalServerError(
                format!("Firewall table '{}' or set '{}' not found. Please run setup script.", self.table, self.set)
            )),
        }
    }

    async fn setup(&self) -> Result<(), AppError> {
        println!("🛡️ Initializing NFTables rules...");

        // 1. Create Table (Idempotent)
        self.run_nft_cmd(&["add", "table", "inet", &self.table])?;

        // 2. Create Chain
        // We set priority 0 to run alongside standard filters.
        // We DO NOT set 'policy drop' because we want to fall through to the user's firewall
        // if it's not our traffic.
        self.run_nft_cmd(&[
            "add", "chain", "inet", &self.table, &self.chain,
            "{ type filter hook input priority 0; }"
        ])?;

        // 3. Flush Chain (Prevents duplicate rules on restart)
        self.run_nft_cmd(&["flush", "chain", "inet", &self.table, &self.chain])?;

        // 4. Create the Dynamic Set
        self.run_nft_cmd(&[
            "add", "set", "inet", &self.table, &self.set,
            "{ type ipv4_addr; flags timeout; }"
        ])?;

        // 5. RULE: Allow API Traffic (Port 3000)
        // "tcp dport 3000 accept"
        let port_str = self.api_port.to_string();
        self.run_nft_cmd(&[
            "add", "rule", "inet", &self.table, &self.chain,
            "tcp", "dport", &port_str, "accept"
        ])?;

        // 6. RULE: Allow Whitelisted IPs
        // "ip saddr @allowed_clients accept"
        let set_ref = format!("@{}", self.set);
        self.run_nft_cmd(&[
            "add", "rule", "inet", &self.table, &self.chain,
            "ip", "saddr", &set_ref, "accept"
        ])?;

        Ok(())
    }
}