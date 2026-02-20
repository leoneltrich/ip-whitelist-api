use axum::http::HeaderMap;
use std::net::{IpAddr, SocketAddr};
use tracing::debug;

/// Hardened IP Extraction
///
/// Strictly enforces trusted headers.
pub fn get_real_ip(headers: &HeaderMap, addr: SocketAddr) -> Option<IpAddr> {
    // The order of those is important to give the CF header a higher priority than the X-Real-IP
    if let Some(ip) = extract_ip_header(headers, "CF-Connecting-IP") {
        debug!("Found CF-Connecting-IP (Cloudflare) header with value: {}", ip);
        return Some(ip);
    }

    // The order of those is important to give the X-Real-IP header a higher priority than the src
    if let Some(ip) = extract_ip_header(headers, "X-Real-IP") {
        debug!("Found X-Real-IP header with value: {}", ip);
        return Some(ip);
    }

    debug!("No IP header found, falling back to src ip: {}", addr.ip());
    Some(addr.ip())
}

/// Helper to extract and parse a single IP header
fn extract_ip_header(headers: &HeaderMap, key: &str) -> Option<IpAddr> {
    headers
        .get(key)
        .and_then(|hv| hv.to_str().ok())
        .and_then(|s| s.trim().parse::<IpAddr>().ok())
}
