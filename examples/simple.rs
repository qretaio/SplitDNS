use splitdns::{DnsManager, ResolverConfig};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ResolverConfig {
        #[cfg(target_os = "linux")]
        interface: "qnet0".to_string(),
        domain: "internal.company.com".to_string(),
        resolvers: vec![
            "192.168.1.10:53".parse::<SocketAddr>()?,
            "10.0.0.5:5353".parse::<SocketAddr>()?,
        ],
    };

    // Add the resolver configuration
    match DnsManager::add_resolver(&config).await {
        Ok(_) => println!("✓ Successfully added resolver configuration"),
        Err(e) => println!("✗ Failed to add resolver: {}", e),
    }

    // Note: In a real application, you might want to keep the configuration
    // For this example, we'll clean it up
    match DnsManager::remove_resolver(&config).await {
        Ok(_) => println!("✓ Successfully removed resolver configuration"),
        Err(e) => println!("✗ Failed to remove resolver: {}", e),
    }
    Ok(())
}
