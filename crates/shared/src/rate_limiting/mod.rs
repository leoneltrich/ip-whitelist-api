use crate::utils::get_real_ip;
use axum::extract::ConnectInfo;
use axum::http::Request;
use std::net::{IpAddr, SocketAddr};
use tower_governor::key_extractor::KeyExtractor;
use tower_governor::GovernorError;
use tracing::error;
use tracing::log::debug;

/// A custom key extractor for tower-governor that uses the shared `get_real_ip` logic.
/// This ensures rate limiting works correctly when the service is behind a reverse proxy (like Nginx).
#[derive(Debug, Clone, Copy, Default)]
pub struct SmartIpExtractor;

impl KeyExtractor for SmartIpExtractor {
    type Key = IpAddr;

    fn extract<B>(&self, req: &Request<B>) -> Result<Self::Key, GovernorError> {
        debug!("Extracting IP address from request headers");
        let headers = req.headers();

        let socket_addr = req
            .extensions()
            .get::<ConnectInfo<SocketAddr>>()
            .map(|ConnectInfo(a)| *a)
            .ok_or_else(|| {
                error!("Could not get source address!");
                GovernorError::UnableToExtractKey
            })?;

        get_real_ip(headers, socket_addr).ok_or_else(|| {
            error!("Could not extract IP address from request headers!");
            GovernorError::UnableToExtractKey
        })
    }
}
