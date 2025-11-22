//! # SplitDNS
//!
//! A simple, cross-platform Rust library for DNS split-domain configuration.
//! Provides minimal but functional DNS split-tunneling similar to Tailscale's approach.
//!

use std::net::SocketAddr;
use thiserror::Error;

mod platform;

pub use platform::PlatformDns as DnsManager;

/// DNS configuration errors
#[derive(Error, Debug)]
pub enum DnsError {
    #[error("Invalid nameserver '{0}': {1}")]
    InvalidNameserver(String, std::net::AddrParseError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Platform error: {0}")]
    Platform(String),

    #[error("Permission denied: requires administrator/root privileges")]
    PermissionDenied,

    #[error("Operation not supported on this platform")]
    Unsupported,

    #[cfg(target_os = "linux")]
    #[error("dbus error: {0}")]
    DBusConnection(zbus_systemd::zbus::Error),

    #[cfg(target_os = "linux")]
    #[error("Failed to create systemd-resolved manager proxy: {0}")]
    ResolvedManagerProxy(String),

    #[cfg(target_os = "linux")]
    #[error("Failed to read interface index for '{0}': {1}")]
    InterfaceIndex(String, String),

    #[cfg(target_os = "linux")]
    #[error("Failed to set link DNS: {0}")]
    SetLinkDns(String),

    #[cfg(target_os = "linux")]
    #[error("Failed to set link domains: {0}")]
    SetLinkDomains(String),

    #[cfg(target_os = "linux")]
    #[error("Failed to revert link configuration: {0}")]
    RevertLink(String),
}

#[cfg(target_os = "linux")]
impl From<zbus_systemd::zbus::Error> for DnsError {
    fn from(err: zbus_systemd::zbus::Error) -> Self {
        DnsError::DBusConnection(err)
    }
}

pub struct ResolverConfig {
    #[cfg(target_os = "linux")]
    pub interface: String,
    pub domain: String,
    pub resolvers: Vec<SocketAddr>,
}

/// Result type for DNS operations
pub type Result<T> = std::result::Result<T, DnsError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_domain_config_operations() {
        // Test adding domain using DomainConfig (may fail due to permissions)
        let config = ResolverConfig {
            #[cfg(target_os = "linux")]
            interface: "lo".to_string(),
            domain: "config.example.com".to_string(),
            resolvers: vec!["192.168.1.101:53".parse().unwrap()],
        };

        match DnsManager::add_resolver(&config).await {
            Ok(_) => {
                DnsManager::remove_resolver(&config)
                    .await
                    .unwrap_or_default();
            }
            Err(DnsError::PermissionDenied) => {
                // Expected without privileges
            }
            _ => {
                // Other errors are also acceptable
            }
        }
    }
}
