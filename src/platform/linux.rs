//! Linux implementation using systemd-resolved D-Bus API

use crate::{DnsError, ResolverConfig, Result};
use std::net::IpAddr;
use zbus_systemd::resolve1::ManagerProxy as ResolvedManagerProxy;
use zbus_systemd::zbus::Connection;

pub const AF_INET: i32 = 2;
pub const AF_INET6: i32 = 10;

/// Linux DNS manager using systemd-resolved D-Bus API
pub struct LinuxDns;

/// Get network interface index from interface name
fn ifindex_from_name(name: &str) -> Result<i32> {
    let path = format!("/sys/class/net/{}/ifindex", name);
    let content = std::fs::read_to_string(&path)
        .map_err(|e| DnsError::InterfaceIndex(name.to_string(), e.to_string()))?;

    content.trim().parse::<i32>().map_err(|_| {
        DnsError::InterfaceIndex(name.to_string(), "Invalid ifindex value".to_string())
    })
}

/// Convert SocketAddr to systemd-resolved format
fn socketaddr_to_systemd_format(addr: std::net::SocketAddr) -> Result<(i32, Vec<u8>)> {
    match addr.ip() {
        IpAddr::V4(ipv4) => Ok((AF_INET, ipv4.octets().to_vec())),
        IpAddr::V6(ipv6) => Ok((AF_INET6, ipv6.octets().to_vec())),
    }
}

impl LinuxDns {
    pub async fn add_resolver(config: &ResolverConfig) -> Result<()> {
        let conn = Connection::system().await?;
        let resolved = ResolvedManagerProxy::new(&conn).await?;

        let ifindex = ifindex_from_name(&config.interface)?;

        // Convert resolver addresses to systemd-resolved format
        let dns_servers: Result<Vec<(i32, Vec<u8>)>> = config
            .resolvers
            .iter()
            .map(|&addr| socketaddr_to_systemd_format(addr))
            .collect();
        let dns_servers = dns_servers?;

        // Set DNS servers for the interface
        resolved
            .set_link_dns(ifindex, dns_servers)
            .await
            .map_err(|e| DnsError::SetLinkDns(e.to_string()))?;

        // Set DNS servers for the interface
        resolved
            .set_link_dns_over_tls(ifindex, "no".to_string())
            .await
            .map_err(|e| DnsError::SetLinkDns(e.to_string()))?;

        // Set domain routing (routing-only = true for split-DNS)
        let domains = vec![(config.domain.clone(), true)];
        resolved
            .set_link_domains(ifindex, domains)
            .await
            .map_err(|e| DnsError::SetLinkDomains(e.to_string()))?;

        Ok(())
    }

    pub async fn remove_resolver(config: &ResolverConfig) -> Result<()> {
        let conn = Connection::system().await?;
        let resolved = ResolvedManagerProxy::new(&conn).await?;

        let ifindex = ifindex_from_name(&config.interface)?;

        resolved
            .revert_link(ifindex)
            .await
            .map_err(|e| DnsError::RevertLink(e.to_string()))?;

        Ok(())
    }
}
