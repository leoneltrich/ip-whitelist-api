use crate::utils::get_real_ip;
use axum::extract::ConnectInfo;
use axum::http::Request;
use std::net::{IpAddr, SocketAddr};
use tower_governor::key_extractor::KeyExtractor;
use tower_governor::GovernorError;

/// A custom key extractor for tower-governor that uses the shared `get_real_ip` logic.
/// This ensures rate limiting works correctly when the service is behind a reverse proxy (like Nginx).
#[derive(Debug, Clone, Copy, Default)]
pub struct SmartIpExtractor;

impl KeyExtractor for SmartIpExtractor {
    type Key = IpAddr;

    fn extract<B>(&self, req: &Request<B>) -> Result<Self::Key, GovernorError> {
        // 1. Get the headers from the request
        let headers = req.headers();

        // 2. Get the peer address from Axum's ConnectInfo extension
        // Note: This requires the app to be served with .into_make_service_with_connect_info::<SocketAddr>()
        let addr = req
            .extensions()
            .get::<ConnectInfo<SocketAddr>>()
            .map(|ConnectInfo(a)| *a)
            .ok_or(GovernorError::UnableToExtractKey)?;

        // 3. Use the shared utility to find the "True" IP
        get_real_ip(headers, addr).ok_or(GovernorError::UnableToExtractKey)
    }
}
